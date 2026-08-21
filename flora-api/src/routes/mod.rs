//! Routes module - all API endpoints.

use axum::{Router, routing::get};

pub mod auth;
pub mod channels;
pub mod files;
pub mod health;
pub mod notifications;
pub mod org;
pub mod rbac;
pub mod search;
pub mod tasks;
pub mod websocket;
pub mod workspace;

use crate::AppState;

/// Creates the main application router with all routes registered.
pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // Health check (no auth required)
        .route("/health", get(health::health_check))
        .route("/health/ready", get(health::ready_check))
        .route("/health/live", get(health::live_check))
        // Auth routes (no auth required for login/callback; auth required for logout/refresh)
        .nest("/auth", auth::create_auth_router())
        // Protected API routes (require auth + X-Organization-ID)
        .nest("/api/v1/organizations", org::create_org_router())
        .nest(
            "/api/v1/workspaces",
            workspace::create_workspace_router().with_state(app_state.clone()),
        )
        .nest("/api/v1/channels", channels::create_channels_router())
        .nest("/api/v1/tasks", tasks::create_tasks_router())
        .nest("/api/v1/files", files::create_files_router())
        .nest("/api/v1/search", search::create_search_router())
        .nest(
            "/api/v1/roles",
            rbac::create_rbac_router().with_state(app_state.clone()),
        )
        .nest(
            "/api/v1/notifications",
            notifications::create_notifications_router(),
        )
        // WebSocket for real-time messaging
        .route("/ws", get(websocket::websocket_handler))
        // Rate limiting
        .layer(axum::middleware::from_fn(
            crate::middleware::rate_limit::rate_limit_middleware,
        ))
        .layer(axum::extract::Extension(std::sync::Arc::new(
            crate::middleware::rate_limit::RateLimiterState::default(),
        )))
        // Wire shared application state to all routes
        .with_state(app_state)
}
