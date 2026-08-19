//! Workspace management routes.

use axum::Router;

use super::AppState;

/// Creates the workspace router.
pub fn create_workspace_router() -> Router<AppState> {
    Router::new()
    // TODO: routes per T020
}
