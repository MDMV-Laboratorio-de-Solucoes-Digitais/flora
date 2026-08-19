//! Request extractors for authentication and organization context.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
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
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Err(axum::http::StatusCode::UNAUTHORIZED)
    }
}
