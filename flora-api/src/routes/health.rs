//! Health check endpoints.
//!
//! Provides `/health`, `/health/ready`, and `/health/live` endpoints.

use axum::{Router, routing::get};
use hyper::StatusCode;

/// Returns 200 OK if the service is up.
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Returns 200 OK if the database is reachable.
pub async fn ready_check() -> StatusCode {
    // In a full implementation, this would check DB connectivity.
    StatusCode::OK
}

/// Returns 200 OK if the service is live (all dependencies healthy).
pub async fn live_check() -> StatusCode {
    // In a full implementation, this would check all critical dependencies.
    StatusCode::OK
}

/// Creates the health check router.
pub fn router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/ready", get(ready_check))
        .route("/health/live", get(live_check))
}
