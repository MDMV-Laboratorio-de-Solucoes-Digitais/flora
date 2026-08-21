//! Rate limiting middleware using Valkey (fallback to in-memory).

use axum::{
    body::Body,
    extract::{ConnectInfo, Extension},
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::warn;

/// State for the in-memory rate limiter.
#[derive(Clone, Debug)]
pub struct RateLimiterState {
    requests: Arc<RwLock<HashMap<IpAddr, Vec<Instant>>>>,
    limit: usize,
    window: Duration,
}

impl RateLimiterState {
    /// Creates a new `RateLimiterState`.
    #[must_use]
    pub fn new(limit: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            limit,
            window: Duration::from_secs(window_secs),
        }
    }

    /// Cleans up expired entries.
    pub async fn cleanup(&self) {
        let now = Instant::now();
        let window = self.window;
        let mut requests = self.requests.write().await;
        
        requests.retain(|_, timestamps| {
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

/// Simple in-memory rate limiter middleware.
///
/// # Errors
///
/// Returns 429 Too Many Requests if rate limit is exceeded.
pub async fn rate_limit_middleware(
    Extension(state): Extension<Arc<RateLimiterState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // Extract IP from headers or fallback to socket addr
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
        .unwrap_or_else(|| addr.ip());

    let now = Instant::now();
    let is_exceeded = {
        let mut requests = state.requests.write().await;
        let timestamps = requests.entry(ip).or_default();
        
        // Remove old timestamps
        timestamps.retain(|&ts| now.duration_since(ts) <= state.window);
        
        let res = if timestamps.len() >= state.limit {
            true
        } else {
            timestamps.push(now);
            false
        };
        drop(requests);
        res
    };
    
    if is_exceeded {
        warn!(ip = %ip, "Rate limit exceeded");
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    // Periodically cleanup (very simple approach)
    // We do a quick probabilistic cleanup here to avoid memory leaks
    if std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).is_ok_and(|d| d.as_millis() % 100 == 0) {
        let state_clone = Arc::clone(&state);
        drop(tokio::spawn(async move {
            state_clone.cleanup().await;
        }));
    }

    Ok(next.run(req).await)
}
