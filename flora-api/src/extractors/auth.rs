//! Request extractors for authentication and organization context.

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};
use uuid::Uuid;

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
        let claims = flora_core::utils::jwt::decode_token(token, &app_state.config.app.jwt_secret)
            .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {e}")))?;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid user ID in token".to_string(),
            )
        })?;

        Ok(Self {
            user_id,
            email: claims.email,
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
        let claims = flora_core::utils::jwt::decode_token(token, &app_state.config.app.jwt_secret)
            .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {e}")))?;

        let organization_id = Uuid::parse_str(&claims.organization_id).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid organization ID in token".to_string(),
            )
        })?;

        let workspace_id = parts
            .headers
            .get("x-workspace-id")
            .and_then(|val| val.to_str().ok())
            .and_then(|val| Uuid::parse_str(val).ok());

        Ok(Self {
            organization_id,
            workspace_id,
        })
    }
}

/// Authenticated user & organization context extracted from JWT Bearer token.
///
/// Validates the JWT signature, expiry, subject (user ID), and scoped organization ID
/// per FR-013 and FR-015.
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

impl<S> FromRequestParts<S> for AuthExtractor
where
    S: Clone + Send + Sync + 'static,
    AppState: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

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
        let claims = flora_core::utils::jwt::decode_token(token, &app_state.config.app.jwt_secret)
            .map_err(|e| (StatusCode::UNAUTHORIZED, format!("Invalid token: {e}")))?;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid user ID in token".to_string(),
            )
        })?;

        let organization_id = Uuid::parse_str(&claims.organization_id).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid organization ID in token".to_string(),
            )
        })?;

        let workspace_id = parts
            .headers
            .get("x-workspace-id")
            .and_then(|val| val.to_str().ok())
            .and_then(|val| Uuid::parse_str(val).ok());

        Ok(Self {
            user_id,
            organization_id,
            email: claims.email.clone(),
            workspace_id,
            claims,
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
            auth_service: std::sync::Arc::new(flora_core::services::AuthService::new()),
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
            search_service: std::sync::Arc::new(
                flora_search::SearchService::new(
                    "http://localhost:7700",
                    None,
                    &config.search.index_template,
                )
                .unwrap_or_else(|_| panic!("search")),
            ),
            messaging_service: None,
        }
    }

    #[tokio::test]
    async fn test_auth_extractor_valid_jwt() {
        let user_id = Uuid::now_v7();
        let org_id = Uuid::now_v7();
        let config = Config::default();
        let claims = FloraClaims::new(
            user_id,
            "jti-1".to_string(),
            org_id,
            "user@flora.test",
            3600,
        );
        let token = encode_token(&claims, &config.app.jwt_secret);

        let req = Request::builder()
            .header("Authorization", format!("Bearer {token}"))
            .header("x-workspace-id", org_id.to_string())
            .body(())
            .unwrap_or_else(|_| Request::new(()));

        let (mut parts, ()) = req.into_parts();
        let state = make_test_app_state(&config);

        let auth_ext = AuthExtractor::from_request_parts(&mut parts, &state).await;
        assert!(auth_ext.is_ok());
        let auth_val = auth_ext.unwrap_or_else(|_| panic!("auth failed"));
        assert_eq!(auth_val.user_id, user_id);
        assert_eq!(auth_val.organization_id, org_id);
        assert_eq!(auth_val.email, "user@flora.test");
        assert_eq!(auth_val.workspace_id, Some(org_id));
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
