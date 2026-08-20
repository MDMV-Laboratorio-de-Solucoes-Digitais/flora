use axum::{
    extract::{FromRequestParts, State},
    http::{StatusCode, request::Parts},
};
use flora_core::{
    config::Config,
    utils::jwt::{self, FloraClaims},
};
use std::ops::Deref;
use uuid::Uuid;

/// Application state that holds connections, services, and configuration.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Database connection pool.
    pub db_pool: flora_core::PgPool,
    /// Application configuration.
    pub config: Arc<Config>,
}

impl AppState {
    /// Creates a new `AppState` with the given database pool and configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the database pool cannot be created.
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let db_pool = flora_core::database::create_pool(&config).await?;
        Ok(Self {
            db_pool: Arc::new(db_pool),
            config: Arc::new(config),
        })
    }
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

impl<S> FromRequestParts<S> for UserContext
where
    S: Clone + Send + Sync + 'static,
    S: Deref<Target = AppState>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the Authorization header
        let token = parts
            .headers
            .get(axum::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                if header.starts_with("Bearer ") {
                    Some(&header[7..])
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid Authorization header".to_string(),
                )
            })?;

        // Decode the token
        let claims = jwt::decode_token(token, state.config.app.jwt_secret.as_bytes())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(UserContext {
            user_id: claims.user_id,
            email: claims.email,
        })
    }
}

impl<S> FromRequestParts<S> for OrgContext
where
    S: Clone + Send + Sync + 'static,
    S: Deref<Target = AppState>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the Authorization header
        let token = parts
            .headers
            .get(axum::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                if header.starts_with("Bearer ") {
                    Some(&header[7..])
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid Authorization header".to_string(),
                )
            })?;

        // Decode the token
        let claims = jwt::decode_token(token, state.config.app.jwt_secret.as_bytes())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(OrgContext {
            organization_id: claims.organization_id,
            workspace_id: None, // Workspace switching is handled separately (see T046)
        })
    }
}

// Re-export AppState for convenience
pub use self::app_state::AppState;

mod app_state {
    use super::*;
}
