//! Rate limiting middleware using Valkey.

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};

/// Simple in-memory rate limiter (placeholder - Valkey-based implementation per T014).
///
/// # Errors
///
/// Returns an error if rate limiting is enforced. Currently always succeeds.
pub async fn rate_limit_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // TODO: Implement per T014 using Valkey
    Ok(next.run(req).await)
}
