//! Database migrations for Flora Workspace.
//!
//! This module contains the schema definitions and migration logic.
//! Migrations are applied using sqlx-cli at build/deploy time.

use crate::error::{Error, Result};

/// Applies all pending migrations to the database.
///
/// This function is typically called at application startup.
///
/// # Errors
///
/// Returns an error if migrations fail to apply.
pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<()> {
    // Apply migrations using sqlx::migrate! macro at runtime
    // This function is a placeholder for potential programmatic migrations
    sqlx::migrate!("./src/migrations/sql")
        .run(pool)
        .await
        .map_err(|err| Error::Migration(err.to_string()))
}

/// Returns the current database schema version.
///
/// # Errors
///
/// Returns an error if the database query fails.
pub async fn get_schema_version(pool: &sqlx::PgPool) -> Result<Option<i32>> {
    let row = sqlx::query_as::<_, (i32,)>(
        "SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}
