//! Request extractors for authentication and organization context.

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use std::ops::Deref;
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
/// TODO: Full implementation with JWT validation per T009.
impl<S> FromRequestParts<S> for UserContext
where
    S: Clone + Send + Sync + 'static,
    S: Deref<Target = AppState>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // TODO: Implement proper JWT validation
        // For now, return an error to indicate unimplemented functionality
        Err((
            StatusCode::UNAUTHORIZED,
            "Authentication not implemented".to_string(),
        ))
    }
}

/// Extracts organization context from the request's headers.
impl<S> FromRequestParts<S> for OrgContext
where
    S: Clone + Send + Sync + 'static,
    S: Deref<Target = AppState>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // TODO: Implement proper JWT validation
        // For now, return an error to indicate unimplemented functionality
        Err((
            StatusCode::UNAUTHORIZED,
            "Organization context not implemented".to_string(),
        ))
    }
}

/// Marker extractor for authentication - used to indicate protected routes
#[derive(Debug, Clone, Copy)]
pub struct AuthExtractor;

// Re-export AppState for use in extractors
pub use crate::state::AppState;
