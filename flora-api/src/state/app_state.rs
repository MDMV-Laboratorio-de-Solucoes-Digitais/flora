// Application state that holds connections, services, and configuration.
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use std::sync::Arc;

use flora_core::PgPool;
use flora_core::config::Config;

/// Application state shared across all requests.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Database connection pool.
    pub db_pool: Arc<PgPool>,
    /// Application configuration.
    pub config: Arc<Config>,
}

impl AppState {
    /// Creates a new `AppState` with the given database pool and configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the database pool cannot be created.
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let db_pool = flora_core::database::create_pool(&config).await?;
        Ok(Self {
            db_pool: Arc::new(db_pool),
            config: Arc::new(config),
        })
    }
}

impl<S> FromRequestParts<S> for AppState
where
    S: Send + Sync + Clone + 'static,
{
    type Rejection = (axum::http::StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // The AppState is typically injected via `.with_state()` in the router,
        // so this extractor is rarely needed directly.
        Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "AppState extractor not available".to_string(),
        ))
    }
}
