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
            .ok_or_else(|| (
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header".to_string(),
            ))?;

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
            (StatusCode::UNAUTHORIZED, "Invalid user ID in token".to_string())
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
            .ok_or_else(|| (
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header".to_string(),
            ))?;

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
            (StatusCode::UNAUTHORIZED, "Invalid organization ID in token".to_string())
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

/// Marker extractor for authentication - used to indicate protected routes
#[derive(Debug, Clone, Copy)]
pub struct AuthExtractor;

// Re-export AppState for use in extractors
pub use crate::state::AppState;
