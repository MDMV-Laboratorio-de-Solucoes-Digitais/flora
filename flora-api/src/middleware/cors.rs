//! CORS middleware configuration.

use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

/// Creates a CORS layer allowing the frontend origin.
#[must_use]
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(86400))
}
