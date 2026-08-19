//! Database connection pool and utilities.

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;
use crate::error::Result;

/// Creates a new `PostgreSQL` connection pool.
///
/// # Errors
///
/// Returns an error if the connection pool cannot be created.
pub async fn create_pool(config: &Config) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database.postgres_url)
        .await
        .map_err(crate::Error::from_sqlx)
}
