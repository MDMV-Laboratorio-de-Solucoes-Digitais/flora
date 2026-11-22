//! Configuration management for Flora Workspace.
//!
//! Supports environment variables, `.env` files, and `config.yaml`.
//! All values are validated at startup.

pub mod app;
pub mod bootstrap;
pub mod database;
pub mod hitl;
pub mod messaging;
pub mod oidc;
pub mod search;
pub mod storage;

use figment::Figment;
use serde::{Deserialize, Serialize};

/// Root configuration for Flora.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    /// Application-level settings.
    pub app: app::AppConfig,
    /// HITL receipt signing; disabled when the secret is empty.
    pub hitl: hitl::HitlConfig,
    /// Startup administrator password configuration.
    pub bootstrap: bootstrap::BootstrapConfig,
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
        let figment = Figment::new()
            .merge(figment::providers::Serialized::defaults(Self::default()))
            .merge(figment::providers::Env::prefixed("FLORA_").split("__"));

        figment.extract::<Self>()
    }

    /// Validate all configuration values.
    ///
    /// # Errors
    ///
    /// Returns an error if any configuration value fails validation.
    pub fn validate(&self) -> Result<(), crate::Error> {
        self.app.validate()?;
        self.bootstrap.validate()?;
        self.hitl.validate()?;
        self.database.validate()?;
        self.oidc.validate()?;
        self.storage.validate()?;
        self.search.validate()?;
        self.messaging.validate()?;
        Ok(())
    }
}
