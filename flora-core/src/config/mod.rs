//! Configuration management for Flora Workspace.
//!
//! Supports environment variables, `.env` files, and `config.yaml`.
//! All values are validated at startup.

pub mod app;
pub mod database;
pub mod messaging;
pub mod notifications;
pub mod oidc;
pub mod search;
pub mod storage;

use figment::Figment;
use serde::Deserialize;

/// Root configuration for Flora.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    /// Application-level settings.
    pub app: app::AppConfig,
    /// Database connection settings.
    pub database: database::DatabaseConfig,
    /// OIDC provider configuration.
    pub oidc: oidc::OidcConfig,
    /// Storage backend configuration.
    pub storage: storage::StorageConfig,
    /// Search engine configuration.
    pub search: search::SearchConfig,
    /// Messaging service configuration.
    pub messaging: messaging::MessagingConfig,
    /// Notification delivery configuration.
    pub notifications: notifications::NotificationsConfig,
}

impl Config {
    /// Load configuration from environment variables and config files.
    ///
    /// # Errors
    ///
    /// Returns an error if configuration values are missing or invalid.
    #[expect(
        clippy::result_large_err,
        reason = "Config loading is a top-level initialization; returning a large error enum is appropriate here."
    )]
    pub fn load() -> figment::Result<Self> {
        let figment = Figment::new().merge(figment::providers::Env::prefixed("FLORA_"));

        figment.extract::<Self>()
    }

    /// Validate all configuration values.
    ///
    /// # Errors
    ///
    /// Returns an error if any configuration value fails validation.
    pub fn validate(&self) -> Result<(), crate::Error> {
        self.app.validate()?;
        self.database.validate()?;
        self.oidc.validate()?;
        self.storage.validate()?;
        self.search.validate()?;
        self.messaging.validate()?;
        self.notifications.validate()?;
        Ok(())
    }
}
