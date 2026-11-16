//! Health check endpoints.
//!
//! Provides `/health`, `/health/ready`, and `/health/live` endpoints.

use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::AppState;

/// Basic liveness/status response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Service status.
    pub status: &'static str,
    /// Optional capability hints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<bool>,
    /// Optional capability hints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messaging: Option<bool>,
}

/// Returns 200 OK if the service process is up.
pub async fn health_check(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok",
            search: Some(state.search_service.is_some()),
            messaging: Some(state.messaging_service.is_some()),
        }),
    )
}

/// Returns 200 OK if required dependencies (database) are reachable.
pub async fn ready_check(State(state): State<AppState>) -> StatusCode {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(state.db_pool.as_ref())
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::warn!(error = %e, "Readiness check failed: database unreachable");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}

/// Returns 200 OK if the service is live.
pub async fn live_check() -> StatusCode {
    StatusCode::OK
}
