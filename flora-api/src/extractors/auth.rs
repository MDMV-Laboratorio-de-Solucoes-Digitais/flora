//! Request extractors for authentication and organization context.

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use chrono::Utc;
use uuid::Uuid;

/// A JWT that has been validated against the server-side session store.
#[derive(Debug, Clone)]
pub(crate) struct VerifiedSession {
    /// The authenticated user's ID.
    pub(crate) user_id: Uuid,
    /// The organization the session is scoped to.
    pub(crate) organization_id: Uuid,
    /// The decoded JWT claims.
    pub(crate) claims: flora_core::utils::jwt::FloraClaims,
}

/// Verifies a raw bearer token and enforces server-side session authority.
///
/// The JWT is decoded and then its `jti` is looked up in PostgreSQL. A session is
/// accepted only when it exists, is active, has not expired, and belongs to the same
/// user and organization as the token claims (including the nil-organization
/// pre-selection session).
///
/// # Errors
///
/// Returns `401` for an invalid token or a missing/revoked/expired/mismatched
/// session, and `500` when the session store is unavailable. Error messages are
/// intentionally generic to avoid leaking session state.
pub(crate) async fn verify_bearer_token(
    token: &str,
    state: &AppState,
) -> Result<VerifiedSession, (StatusCode, String)> {
    let claims = flora_core::utils::jwt::decode_token(token, &state.config.app.jwt_secret)
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired session".to_string(),
            )
        })?;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Invalid or expired session".to_string(),
        )
    })?;
    let organization_id = Uuid::parse_str(&claims.organization_id).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Invalid or expired session".to_string(),
        )
    })?;

    let session = state
        .session_service
        .validate_session(&claims.jti)
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "session lookup failed during authentication");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication service unavailable".to_string(),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired session".to_string(),
            )
        })?;

    if !session.is_active
        || session.expires_at <= Utc::now()
        || session.user_id != user_id
        || session.organization_id != organization_id
    {
        tracing::debug!(
            user_id = %user_id,
            jti = %claims.jti,
            "session rejected by server-side authority"
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid or expired session".to_string(),
        ));
    }

    Ok(VerifiedSession {
        user_id,
        organization_id,
        claims,
    })
}

/// Authenticates a `Bearer` token from request headers.
///
/// # Errors
///
/// Returns `401` for a missing/invalid token or a missing/revoked/expired/mismatched
/// session, and `500` when the session store is unavailable.
async fn authenticate_bearer(
    parts: &Parts,
    state: &AppState,
) -> Result<VerifiedSession, (StatusCode, String)> {
    let auth_header = parts
        .headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header".to_string(),
            )
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Authorization header must start with Bearer".to_string(),
        ));
    }

    let token = &auth_header["Bearer ".len()..];
    verify_bearer_token(token, state).await
}

/// Organization context extracted from the session.
#[derive(Clone, Debug)]
pub struct OrgContext {
    /// The organization the authenticated user belongs to.
    pub organization_id: Uuid,
    /// The active workspace within the organization (None if not yet selected).
    pub workspace_id: Option<Uuid>,
}

/// User identity extracted from JWT/session.
#[derive(Clone, Debug)]
pub struct UserContext {
    /// The unique user identifier.
    pub user_id: Uuid,
    /// The user's email address.
    pub email: String,
}

/// Extracts user context from the request's Authorization header.
impl<S> FromRequestParts<S> for UserContext
where
    S: Clone + Send + Sync + 'static,
    AppState: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let verified = authenticate_bearer(parts, &app_state).await?;

        Ok(Self {
            user_id: verified.user_id,
            email: verified.claims.email,
        })
    }
}

/// Extracts organization context from the request's headers.
impl<S> FromRequestParts<S> for OrgContext
where
    S: Clone + Send + Sync + 'static,
    AppState: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let verified = authenticate_bearer(parts, &app_state).await?;

        let workspace_id = parts
            .headers
            .get("x-workspace-id")
            .and_then(|val| val.to_str().ok())
            .and_then(|val| Uuid::parse_str(val).ok());

        Ok(Self {
            organization_id: verified.organization_id,
            workspace_id,
        })
    }
}

/// Authenticated user & organization context extracted from JWT Bearer token.
///
/// Validates the JWT signature, expiry, subject (user ID), and scoped organization ID
/// per FR-013 and FR-015, and enforces server-side session authority via the `jti`
/// session row (cycle 012).
#[derive(Debug, Clone)]
pub struct AuthExtractor {
    /// The authenticated user's ID.
    pub user_id: Uuid,
    /// The organization ID the session is scoped to.
    pub organization_id: Uuid,
    /// The user's email address.
    pub email: String,
    /// The active workspace within the organization (if provided in `x-workspace-id` header).
    pub workspace_id: Option<Uuid>,
    /// The decoded JWT claims.
    pub claims: flora_core::utils::jwt::FloraClaims,
}

impl AuthExtractor {
    /// Ensures the token is scoped to a concrete organization (not the nil placeholder).
    ///
    /// # Errors
    ///
    /// Returns `(400 Bad Request, message)` when the session is not bound to a concrete organization.
    pub fn require_org(&self) -> Result<Uuid, (StatusCode, String)> {
        if self.organization_id.is_nil() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Organization context required; select an organization first".to_string(),
            ));
        }
        Ok(self.organization_id)
    }
}

impl<S> FromRequestParts<S> for AuthExtractor
where
    S: Clone + Send + Sync + 'static,
    AppState: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let verified = authenticate_bearer(parts, &app_state).await?;

        let workspace_id = parts
            .headers
            .get("x-workspace-id")
            .and_then(|val| val.to_str().ok())
            .and_then(|val| Uuid::parse_str(val).ok());

        Ok(Self {
            user_id: verified.user_id,
            organization_id: verified.organization_id,
            email: verified.claims.email.clone(),
            workspace_id,
            claims: verified.claims,
        })
    }
}

// Re-export AppState for use in extractors
pub use crate::state::AppState;

#[cfg(test)]
#[expect(
    clippy::panic,
    clippy::unwrap_used,
    reason = "Test setup and assertions"
)]
mod tests {
    use super::*;
    use axum::extract::FromRequestParts;
    use axum::http::Request;
    use flora_core::config::Config;
    use flora_core::utils::jwt::{FloraClaims, encode_token};

    fn make_test_app_state(config: &Config) -> AppState {
        let pool = sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
            .unwrap_or_else(|_| panic!("pool creation failed"));
        AppState {
            db_pool: std::sync::Arc::new(pool),
            config: std::sync::Arc::new(config.clone()),
            rbac_service: std::sync::Arc::new(flora_core::services::RbacService::new(
                std::sync::Arc::new(flora_core::repositories::PgRoleRepository::new(
                    sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                        .unwrap_or_else(|_| panic!("pool")),
                )),
                std::sync::Arc::new(flora_core::repositories::PgMembershipRepository::new(
                    sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                        .unwrap_or_else(|_| panic!("pool")),
                )),
            )),
            session_service: std::sync::Arc::new(flora_core::services::SessionService::new(
                std::sync::Arc::new(flora_core::repositories::PgSessionRepository::new(
                    sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                        .unwrap_or_else(|_| panic!("pool")),
                )),
            )),
            organization_service: std::sync::Arc::new(
                flora_core::services::OrganizationService::new(
                    std::sync::Arc::new(flora_core::repositories::PgOrganizationRepository::new(
                        sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                            .unwrap_or_else(|_| panic!("pool")),
                    )),
                    std::sync::Arc::new(flora_core::repositories::PgMembershipRepository::new(
                        sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                            .unwrap_or_else(|_| panic!("pool")),
                    )),
                    std::sync::Arc::new(flora_core::repositories::PgRoleRepository::new(
                        sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                            .unwrap_or_else(|_| panic!("pool")),
                    )),
                ),
            ),
            workspace_service: std::sync::Arc::new(flora_core::services::WorkspaceService::new(
                std::sync::Arc::new(flora_core::repositories::PgWorkspaceRepository::new(
                    sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                        .unwrap_or_else(|_| panic!("pool")),
                )),
            )),
            channel_service: std::sync::Arc::new(flora_core::services::ChannelService::new(
                std::sync::Arc::new(flora_core::repositories::PgChannelRepository::new(
                    sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                        .unwrap_or_else(|_| panic!("pool")),
                )),
            )),
            message_service: std::sync::Arc::new(flora_core::services::MessageService::new(
                std::sync::Arc::new(flora_core::repositories::PgMessageRepository::new(
                    sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                        .unwrap_or_else(|_| panic!("pool")),
                )),
            )),
            task_service: std::sync::Arc::new(flora_core::services::TaskService::new(
                std::sync::Arc::new(flora_core::repositories::PgTaskRepository::new(
                    sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                        .unwrap_or_else(|_| panic!("pool")),
                )),
            )),
            file_service: std::sync::Arc::new(flora_core::services::FileService::new(
                std::sync::Arc::new(flora_core::repositories::PgFileRepository::new(
                    sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                        .unwrap_or_else(|_| panic!("pool")),
                )),
                std::sync::Arc::new(flora_core::config::storage::StorageConfig::default()),
            )),
            notification_service: std::sync::Arc::new(
                flora_core::services::NotificationService::new(std::sync::Arc::new(
                    flora_core::repositories::PgNotificationRepository::new(
                        sqlx::PgPool::connect_lazy("postgresql://localhost/flora_test")
                            .unwrap_or_else(|_| panic!("pool")),
                    ),
                )),
            ),
            search_service: None,
            messaging_service: None,
            valkey: None,
            oidc_transactions: crate::state::oidc_transactions::OidcTransactionStore::new(None),
            realtime_hub: crate::realtime::create_hub(),
        }
    }

    #[tokio::test]
    async fn test_auth_extractor_invalid_signature_rejected_without_db() {
        let user_id = Uuid::now_v7();
        let org_id = Uuid::now_v7();
        let config = Config::default();
        let claims = FloraClaims::new(
            user_id,
            "jti-invalid-signature".to_string(),
            org_id,
            "user@flora.test",
            3600,
        );
        let token = encode_token(&claims, "not-the-configured-secret");

        let req = Request::builder()
            .header("Authorization", format!("Bearer {token}"))
            .body(())
            .unwrap_or_else(|_| Request::new(()));

        let (mut parts, ()) = req.into_parts();
        let state = make_test_app_state(&config);

        let auth_ext = AuthExtractor::from_request_parts(&mut parts, &state).await;
        assert!(auth_ext.is_err());
        let (status, msg) = auth_ext.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(msg, "Invalid or expired session");
    }

    #[tokio::test]
    async fn test_verify_bearer_token_invalid_signature_rejected_without_db() {
        let user_id = Uuid::now_v7();
        let org_id = Uuid::now_v7();
        let config = Config::default();
        let claims = FloraClaims::new(
            user_id,
            "jti-invalid-signature".to_string(),
            org_id,
            "user@flora.test",
            3600,
        );
        let token = encode_token(&claims, "not-the-configured-secret");
        let state = make_test_app_state(&config);

        let result = verify_bearer_token(&token, &state).await;
        assert!(result.is_err());
        let (status, msg) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(msg, "Invalid or expired session");
    }

    #[tokio::test]
    async fn test_auth_extractor_missing_header_rejected() {
        let config = Config::default();
        let req = Request::builder()
            .body(())
            .unwrap_or_else(|_| Request::new(()));
        let (mut parts, ()) = req.into_parts();
        let state = make_test_app_state(&config);

        let auth_ext = AuthExtractor::from_request_parts(&mut parts, &state).await;
        assert!(auth_ext.is_err());
        let (status, msg) = auth_ext.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(msg.contains("Missing"));
    }
}
