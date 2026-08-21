//! Rate limiting middleware using Valkey with in-memory fallback.
//!
//! Per T014 & FR-020:
//! - 5 login attempts/minute per IP
//! - 20 token refreshes/minute per user/IP
//! - 10 invite acceptances/hour per IP
//! - 100 requests/minute default for general endpoints

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use redis::aio::ConnectionManager;
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::warn;

/// State for the rate limiter supporting Valkey (Redis) backend with in-memory fallback.
#[derive(Clone)]
pub struct RateLimiterState {
    redis: Option<ConnectionManager>,
    in_memory: Arc<RwLock<HashMap<String, Vec<Instant>>>>,
    default_limit: usize,
    default_window: Duration,
}

impl std::fmt::Debug for RateLimiterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RateLimiterState")
            .field("default_limit", &self.default_limit)
            .field("default_window", &self.default_window)
            .field("has_valkey", &self.redis.is_some())
            .finish_non_exhaustive()
    }
}

impl RateLimiterState {
    /// Creates a new in-memory `RateLimiterState`.
    #[must_use]
    pub fn new(limit: usize, window_secs: u64) -> Self {
        Self {
            redis: None,
            in_memory: Arc::new(RwLock::new(HashMap::new())),
            default_limit: limit,
            default_window: Duration::from_secs(window_secs),
        }
    }

    /// Creates a new `RateLimiterState` with an optional Valkey connection manager.
    #[must_use]
    pub fn with_optional_valkey(
        valkey: Option<ConnectionManager>,
        limit: usize,
        window_secs: u64,
    ) -> Self {
        Self {
            redis: valkey,
            in_memory: Arc::new(RwLock::new(HashMap::new())),
            default_limit: limit,
            default_window: Duration::from_secs(window_secs),
        }
    }

    /// Checks if a request for the given key is allowed under the rate limit.
    /// Returns `true` if allowed, `false` if exceeded.
    pub async fn check_rate_limit(&self, key: &str, limit: usize, window_secs: u64) -> bool {
        // Try Valkey first if configured
        if let Some(ref conn) = self.redis {
            let mut conn = conn.clone();
            let redis_key = format!("ratelimit:{key}");
            let count_res: redis::RedisResult<i64> = redis::cmd("INCR")
                .arg(&redis_key)
                .query_async(&mut conn)
                .await;

            if let Ok(count) = count_res {
                if count == 1 {
                    let _: redis::RedisResult<()> = redis::cmd("EXPIRE")
                        .arg(&redis_key)
                        .arg(window_secs)
                        .query_async(&mut conn)
                        .await;
                }
                return count <= i64::try_from(limit).unwrap_or(i64::MAX);
            }
            warn!("Valkey rate limit check failed, falling back to in-memory store");
        }

        // In-memory sliding window check
        let now = Instant::now();
        let window = Duration::from_secs(window_secs);
        let mut map = self.in_memory.write().await;
        let timestamps = map.entry(key.to_string()).or_default();

        timestamps.retain(|&ts| now.duration_since(ts) <= window);

        let exceeded = timestamps.len() >= limit;
        if !exceeded {
            timestamps.push(now);
        }
        drop(map);
        !exceeded
    }

    /// Cleans up expired in-memory entries.
    pub async fn cleanup(&self) {
        let now = Instant::now();
        let window = self.default_window;
        let mut map = self.in_memory.write().await;

        map.retain(|_, timestamps| {
            timestamps.retain(|&ts| now.duration_since(ts) <= window);
            !timestamps.is_empty()
        });
    }
}

impl Default for RateLimiterState {
    fn default() -> Self {
        Self::new(100, 60)
    }
}

/// Rate limiting middleware evaluating endpoint-specific limits per FR-020.
///
/// # Errors
///
/// Returns 429 Too Many Requests if rate limit is exceeded.
pub async fn rate_limit_middleware(
    Extension(state): Extension<Arc<RateLimiterState>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // Extract IP from headers or fallback
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<IpAddr>().ok())
        })
        .unwrap_or_else(|| IpAddr::from([127, 0, 0, 1]));

    let path = req.uri().path();

    // Determine limit and window based on endpoint (FR-020)
    let (key, limit, window_secs) = if path.starts_with("/auth/login") {
        (format!("login:{ip}"), 5, 60)
    } else if path.starts_with("/auth/refresh") {
        let user_key = req
            .headers()
            .get("x-user-id")
            .and_then(|v| v.to_str().ok())
            .map_or_else(|| format!("refresh:{ip}"), |u| format!("refresh:user:{u}"));
        (user_key, 20, 60)
    } else if path.contains("/invites") || path.contains("/invitations") {
        (format!("invite:{ip}"), 10, 3600)
    } else {
        (
            format!("gen:{ip}"),
            state.default_limit,
            state.default_window.as_secs(),
        )
    };

    let allowed = state.check_rate_limit(&key, limit, window_secs).await;

    if !allowed {
        warn!(ip = %ip, path = %path, key = %key, "Rate limit exceeded");
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_rate_limiter_allows_under_limit() {
        let limiter = RateLimiterState::new(3, 60);
        assert!(limiter.check_rate_limit("test-key-1", 3, 60).await);
        assert!(limiter.check_rate_limit("test-key-1", 3, 60).await);
        assert!(limiter.check_rate_limit("test-key-1", 3, 60).await);
        // Exceed limit
        assert!(!limiter.check_rate_limit("test-key-1", 3, 60).await);
    }

    #[tokio::test]
    async fn test_in_memory_rate_limiter_distinct_keys() {
        let limiter = RateLimiterState::new(1, 60);
        assert!(limiter.check_rate_limit("user-1", 1, 60).await);
        assert!(!limiter.check_rate_limit("user-1", 1, 60).await);
        assert!(limiter.check_rate_limit("user-2", 1, 60).await);
    }
}
