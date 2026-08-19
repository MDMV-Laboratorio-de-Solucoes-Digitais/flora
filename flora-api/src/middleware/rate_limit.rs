//! Rate limiting middleware using Valkey.

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};

/// Simple in-memory rate limiter (placeholder - Valkey-based implementation per T014).
pub async fn rate_limit_middleware(
    _req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // TODO: Implement per T014 using Valkey
    Ok(next.run(_req).await)
}
