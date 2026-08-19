//! Database migrations for Flora Workspace.
//!
//! This module contains the schema definitions and migration logic.
//! Migrations are applied using sqlx-cli at build/deploy time.

use crate::error::Result;

/// Applies all pending migrations to the database.
///
/// This function is typically called at application startup.
///
/// # Errors
///
/// Returns an error if migrations fail to apply.
pub fn run_migrations(_pool: &sqlx::PgPool) -> Result<()> {
    // The actual migration .sql files are applied by sqlx-cli
    // This function is a placeholder for potential programmatic migrations
    Ok(())
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
