//! Database connection pool and utilities.

use std::time::Duration;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;
use crate::error::Result;

/// Creates a new `PostgreSQL` connection pool using configured settings.
///
/// # Errors
///
/// Returns an error if the connection pool cannot be created.
pub async fn create_pool(config: &Config) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .acquire_timeout(Duration::from_secs(config.database.acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(config.database.idle_lifetime_secs))
        .connect(&config.database.postgres_url)
        .await
        .map_err(crate::Error::from_sqlx)
}
