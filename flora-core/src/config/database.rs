//! Database configuration (`PostgreSQL` primary).

use serde::{Deserialize, Serialize};

/// Configuration for database connections.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    /// `PostgreSQL` connection string.
    pub postgres_url: String,
    /// `SQLite` database path (reserved; runtime is currently PostgreSQL-only).
    pub sqlite_path: Option<String>,
    /// Maximum number of connections in the pool (conservative default for Seed VPS).
    pub max_connections: u32,
    /// Minimum idle connections.
    pub min_connections: u32,
    /// Connection acquire timeout in seconds.
    pub acquire_timeout_secs: u64,
    /// Idle lifetime in seconds.
    pub idle_lifetime_secs: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            postgres_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://flora:flora@localhost/flora".to_owned()),
            sqlite_path: std::env::var("SQLITE_PATH").ok(),
            max_connections: 10,
            min_connections: 2,
            acquire_timeout_secs: 30,
            idle_lifetime_secs: 600,
        }
    }
}

impl DatabaseConfig {
    /// Validates the database configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.postgres_url.is_empty() && self.sqlite_path.is_none() {
            return Err(crate::Error::Configuration(
                "at least one database URL must be configured (postgres_url or sqlite_path)"
                    .to_owned(),
            ));
        }
        if self.postgres_url.is_empty() {
            return Err(crate::Error::Configuration(
                "database.postgres_url is required (SQLite runtime is not supported)".to_owned(),
            ));
        }
        if self.max_connections == 0 {
            return Err(crate::Error::Configuration(
                "max_connections must be > 0".to_owned(),
            ));
        }
        if self.min_connections > self.max_connections {
            return Err(crate::Error::Configuration(
                "min_connections must be <= max_connections".to_owned(),
            ));
        }
        if self.acquire_timeout_secs == 0 {
            return Err(crate::Error::Configuration(
                "acquire_timeout_secs must be > 0".to_owned(),
            ));
        }
        Ok(())
    }

    /// Returns true if `SQLite` mode is requested (not a supported runtime path).
    #[must_use]
    pub const fn is_sqlite(&self) -> bool {
        self.sqlite_path.is_some()
    }
}
