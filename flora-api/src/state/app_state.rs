//! Application state shared across all request handlers.

use flora_core::error::{Error, Result};
use redis::aio::ConnectionManager;

/// Application state shared across all request handlers.
#[derive(Clone)]
pub struct AppState {
    pub config: flora_core::config::Config,
    pub db_pool: sqlx::PgPool,
    pub redis_manager: ConnectionManager,
}

impl AppState {
    /// Creates a new AppState by connecting to PostgreSQL and Valkey.
    pub async fn new(config: flora_core::config::Config) -> Result<Self> {
        let db_pool = sqlx::PgPool::connect(&config.database.postgres_url)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        // redis 1.6: ConnectionManager::new(client) takes a Client directly.
        // The manager internally manages the multiplexed connection pool.
        let redis_client = redis::Client::open(config.messaging.valkey_url.clone())
            .map_err(|e| Error::Database(e.to_string()))?;
        let redis_manager = ConnectionManager::new(redis_client)
            .await
            .map_err(|e| Error::Messaging(e.to_string()))?;

        Ok(Self {
            config,
            db_pool,
            redis_manager,
        })
    }
}
