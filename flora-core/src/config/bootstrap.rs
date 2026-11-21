//! Startup bootstrap configuration for the local admin password.
//!
//! `FLORA_BOOTSTRAP__ADMIN_EMAIL` and `FLORA_BOOTSTRAP__ADMIN_PASSWORD` are
//! applied at startup only to an existing user whose `password_hash` is NULL.
//! The secret is never logged.

use std::fmt;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::repositories::PgUserRepository;
use crate::services::PasswordService;
use crate::traits::UserRepository;

/// Optional startup credentials for an already-bootstrapped administrator.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct BootstrapConfig {
    /// E-mail of the existing administrator account.
    #[serde(default)]
    pub admin_email: Option<String>,
    /// Initial local password; never serialized or logged.
    #[serde(default, skip_serializing)]
    pub admin_password: Option<String>,
}

impl fmt::Debug for BootstrapConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BootstrapConfig")
            .field("admin_email", &self.admin_email)
            .field(
                "admin_password",
                &self.admin_password.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

impl BootstrapConfig {
    /// Whether both startup credentials were provided.
    #[must_use]
    pub const fn is_configured(&self) -> bool {
        self.admin_email.is_some() && self.admin_password.is_some()
    }

    /// Validates the startup credentials without revealing their values.
    ///
    /// # Errors
    ///
    /// Returns a configuration error for incomplete credentials, an invalid
    /// e-mail, or a password outside 15..1024 characters.
    pub fn validate(&self) -> Result<()> {
        match (self.admin_email.as_deref(), self.admin_password.as_deref()) {
            (None, None) => Ok(()),
            (Some(_), None) => Err(Error::Configuration(
                "FLORA_BOOTSTRAP__ADMIN_PASSWORD is required when ADMIN_EMAIL is set".to_owned(),
            )),
            (None, Some(_)) => Err(Error::Configuration(
                "FLORA_BOOTSTRAP__ADMIN_EMAIL is required when ADMIN_PASSWORD is set".to_owned(),
            )),
            (Some(email), Some(password)) => {
                let email = email.trim();
                let valid = email.len() <= 255
                    && !email.chars().any(char::is_whitespace)
                    && email.split_once('@').is_some_and(|(local, domain)| {
                        !local.is_empty() && !domain.is_empty() && domain.contains('.')
                    });
                if !valid {
                    return Err(Error::Configuration(
                        "FLORA_BOOTSTRAP__ADMIN_EMAIL must be a valid e-mail address".to_owned(),
                    ));
                }
                PasswordService::validate_password(password, "admin_password")
            }
        }
    }
}

/// Applies the configured password to an existing user without a hash.
///
/// # Errors
///
/// Returns an error for invalid configuration or database failure.
pub async fn apply_startup_password(
    pool: &PgPool,
    config: &BootstrapConfig,
    password_service: &PasswordService,
) -> Result<bool> {
    config.validate()?;
    let (Some(email), Some(password)) = (
        config.admin_email.as_deref(),
        config.admin_password.as_deref(),
    ) else {
        return Ok(false);
    };
    let email = email.trim().to_lowercase();

    let repo = PgUserRepository::new(pool.clone());
    let Some(user) = repo.find_by_email(&email).await? else {
        tracing::warn!(
            "FLORA_BOOTSTRAP__ADMIN_EMAIL does not match an existing user; startup password skipped"
        );
        return Ok(false);
    };
    if user.password_hash.is_some() {
        return Ok(false);
    }

    let hash = password_service.hash(password)?;
    let applied = repo
        .set_password_hash_if_absent(user.id, &hash, Utc::now())
        .await?;
    if applied {
        tracing::info!(user_id = %user.id, "Startup administrator password applied");
    }
    Ok(applied)
}

#[cfg(test)]
mod tests {
    use super::BootstrapConfig;

    #[test]
    fn empty_configuration_is_valid() {
        assert!(BootstrapConfig::default().validate().is_ok());
        assert!(!BootstrapConfig::default().is_configured());
    }

    #[test]
    fn partial_configuration_is_rejected() {
        let config = BootstrapConfig {
            admin_email: Some("admin@example.test".to_owned()),
            ..BootstrapConfig::default()
        };
        assert!(config.validate().is_err());
        let config = BootstrapConfig {
            admin_password: Some("SenhaForte@12345".to_owned()),
            ..BootstrapConfig::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn valid_configuration_is_accepted() {
        let config = BootstrapConfig {
            admin_email: Some("admin@example.test".to_owned()),
            admin_password: Some("SenhaForte@12345".to_owned()),
        };
        assert!(config.is_configured());
        assert!(config.validate().is_ok());
        assert!(!format!("{config:?}").contains("SenhaForte@12345"));
    }
}
