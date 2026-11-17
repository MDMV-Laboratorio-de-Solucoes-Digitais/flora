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

/// Machine-readable Markdown manifest for AI agents and crawlers (llms.txt).
const LLMS_TXT_MANIFEST: &str = "\
# Flora Workspace (flora-api)

> Unified collaboration workspace API for organizations, workspaces, channels, tasks, files, search, RBAC, and notifications.

## Health & Discovery
- `GET /health`: Service liveness and capability hints (`search`, `messaging`)
- `GET /health/ready`: Database readiness probe
- `GET /health/live`: Process liveness probe
- `GET /metrics`: Prometheus-compatible process and service metrics
- `GET /llms.txt`: This machine-readable service manifest

## API Endpoints
- `POST /api/v1/auth/*`: Authentication, login, callback, refresh, and logout
- `/api/v1/organizations`: Organization management (requires a verified session Bearer token)
- `/api/v1/workspaces`: Workspace management (requires a verified session Bearer token)
- `/api/v1/channels`: Messaging channels (requires a verified session Bearer token)
- `/api/v1/tasks`: Task management (requires a verified session Bearer token)
- `/api/v1/files`: File storage and retrieval (requires a verified session Bearer token)
- `/api/v1/search`: Full-text search across workspace entities (requires a verified session Bearer token)
- `/api/v1/roles`: Role-based access control (requires a verified session Bearer token)
- `/api/v1/notifications`: User notifications (requires a verified session Bearer token)

Organization context is derived from the verified session (`require_org()`), never from
a request header. `X-Organization-ID` is accepted in CORS only for legacy compatibility;
no route uses it for authorization.
- `GET /ws`, `GET /api/v1/ws`: WebSocket real-time messaging
";

/// Returns a deterministic `text/plain; charset=utf-8` Markdown manifest for `/llms.txt`.
pub async fn llms_txt() -> (
    StatusCode,
    [(axum::http::header::HeaderName, &'static str); 1],
    &'static str,
) {
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; charset=utf-8",
        )],
        LLMS_TXT_MANIFEST,
    )
}

/// Prometheus-compatible metrics payload for `/metrics`.
const PROMETHEUS_METRICS: &str = "\
# HELP flora_api_up Service process up indicator.
# TYPE flora_api_up gauge
flora_api_up 1
# HELP flora_api_info Static service metadata.
# TYPE flora_api_info gauge
flora_api_info{service=\"flora-api\"} 1
";

/// Returns `200 OK` with Prometheus text exposition metrics.
pub async fn metrics() -> (
    StatusCode,
    [(axum::http::header::HeaderName, &'static str); 1],
    &'static str,
) {
    (
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        PROMETHEUS_METRICS,
    )
}

#[cfg(test)]
#[expect(
    clippy::expect_used,
    reason = "test-only body decoding: a failure here is a test bug, not a production path"
)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn test_llms_txt_returns_ok_markdown_manifest() {
        let response = llms_txt().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok()),
            Some("text/plain; charset=utf-8")
        );

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let body = std::str::from_utf8(&body_bytes).expect("utf-8 body");
        assert!(body.contains("# Flora Workspace (flora-api)"));
        assert!(body.contains("/health"));
        assert!(body.contains("/metrics"));
        assert!(body.contains("/api/v1/workspaces"));
    }

    #[tokio::test]
    async fn test_llms_txt_does_not_advertise_header_as_auth() {
        let response = llms_txt().await.into_response();
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let body = std::str::from_utf8(&body_bytes).expect("utf-8 body");
        // No route reads X-Organization-ID; tenancy comes from the verified session.
        assert!(!body.contains("requires auth + `X-Organization-ID`"));
        assert!(body.contains("verified session Bearer token"));
        assert!(body.contains("derived from the verified session"));
    }

    #[tokio::test]
    async fn test_metrics_returns_ok_prometheus_text() {
        let response = metrics().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok()),
            Some("text/plain; version=0.0.4; charset=utf-8")
        );

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        let body = std::str::from_utf8(&body_bytes).expect("utf-8 body");
        assert!(body.contains("flora_api_up 1"));
        assert!(body.contains("flora_api_info{service=\"flora-api\"} 1"));
    }
}
