//! Routes module - all API endpoints.
#![deny(clippy::pedantic)]

use axum::{routing::get, Router};

pub mod auth;
pub mod channels;
pub mod files;
pub mod health;
pub mod notifications;
pub mod org;
pub mod search;
pub mod tasks;
pub mod workspace;

/// Creates the main application router with all routes registered.
#[must_use]
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/health/ready", get(health::ready_check))
        .route("/health/live", get(health::live_check))
        .nest("/auth", auth::create_auth_router())
        .nest("/api/orgs", org::create_org_router())
        .nest("/api/workspaces", workspace::create_workspace_router())
        .nest("/api/channels", channels::create_channels_router())
        .nest("/api/tasks", tasks::create_tasks_router())
        .nest("/api/files", files::create_files_router())
        .nest("/api/search", search::create_search_router())
        .nest(
            "/api/notifications",
            notifications::create_notifications_router(),
        )
}
