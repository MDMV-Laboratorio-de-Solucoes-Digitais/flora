//! OIDC / Zitadel configuration.

use serde::Deserialize;

/// Configuration for OIDC / Zitadel authentication.
#[derive(Debug, Clone, Deserialize)]
pub struct OidcConfig {
    /// Zitadel issuer URL (e.g., <https://zitadel.example.com>).
    pub issuer_url: String,
    /// Zitadel client ID.
    pub client_id: String,
    /// Zitadel client secret.
    pub client_secret: String,
    /// `OAuth2` scopes to request (default: [`openid`, `profile`, `email`]).
    pub scopes: Vec<String>,
    /// PKCE code challenge method (default: "S256").
    pub code_challenge_method: String,
    /// Local fallback enabled when Zitadel is unavailable (default: true in dev).
    pub local_fallback_enabled: bool,
}

impl Default for OidcConfig {
    fn default() -> Self {
        Self {
            issuer_url: std::env::var("ZITADEL_ISSUER_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_owned()),
            client_id: std::env::var("ZITADEL_CLIENT_ID")
                .unwrap_or_else(|_| "flora@localhost".to_owned()),
            client_secret: std::env::var("ZITADEL_CLIENT_SECRET").unwrap_or_default(),
            scopes: vec![
                "openid".to_owned(),
                "profile".to_owned(),
                "email".to_owned(),
            ],
            code_challenge_method: "S256".to_owned(),
            local_fallback_enabled: true,
        }
    }
}

impl OidcConfig {
    /// Validates the OIDC configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    /// Validates the OIDC configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.issuer_url.is_empty() {
            return Err(crate::Error::Configuration(
                "oidc.issuer_url is required".to_owned(),
            ));
        }
        if self.client_id.is_empty() {
            return Err(crate::Error::Configuration(
                "oidc.client_id is required".to_owned(),
            ));
        }
        if self.scopes.is_empty() {
            return Err(crate::Error::Configuration(
                "at least one OIDC scope is required".to_owned(),
            ));
        }
        Ok(())
    }
}
