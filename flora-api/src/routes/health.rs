//! Health check endpoints for Kubernetes/load balancer probes.

use axum::http::StatusCode;
use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

/// Basic health check - always returns 200 if the service is running.
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

/// Readiness check - verifies all dependencies are available.
pub async fn ready_check() -> StatusCode {
    // TODO: Check DB, Valkey, Meilisearch connectivity
    StatusCode::OK
}

/// Liveness check - simple ping to detect hung processes.
pub async fn live_check() -> StatusCode {
    StatusCode::OK
}
