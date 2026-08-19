//! CORS middleware configuration.

use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

/// Creates a CORS layer allowing the frontend origin.
///
/// # Examples
///
/// ```ignore
/// let cors = cors_layer();
/// ```
#[must_use = "CORS layer must be applied to the router"]
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(Duration::from_hours(24))
}
