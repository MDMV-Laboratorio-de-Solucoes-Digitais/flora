//! Application-level configuration.

use serde::{Deserialize, Serialize};

/// Configuration for application-level settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    /// Application name (default: "Flora Workspace")
    pub name: String,
    /// Environment: development, staging, production
    pub env: String,
    /// HTTP host to bind (default: "0.0.0.0")
    pub host: String,
    /// HTTP port to bind (default: 3000)
    pub port: u16,
    /// Base URL for OAuth redirects (e.g., <https://flora.example.com>)
    pub base_url: String,
    /// CORS allowed origins (exact `scheme://host[:port]` strings; Figment array syntax)
    pub cors_origins: Vec<String>,
    /// JWT secret for signing session tokens
    pub jwt_secret: String,
    /// Session token expiry in seconds (default: 3600 = 1 hour)
    pub session_ttl_secs: u64,
    /// Grace period for permission propagation in seconds (default: 5)
    pub permission_propagation_grace_secs: u64,
    /// Default retention period in days (default: 90)
    pub default_retention_days: u32,
    /// Maximum file upload size in bytes (default: 100 MB)
    pub max_upload_bytes: u64,
    /// Allowed file MIME types (glob patterns)
    pub allowed_file_types: Vec<String>,
    /// Whether to enable OpenTelemetry tracing
    pub otel_enabled: bool,
    /// OpenTelemetry collector endpoint
    pub otel_endpoint: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "Flora Workspace".to_owned(),
            env: "development".to_owned(),
            host: "0.0.0.0".to_owned(),
            port: 3000,
            base_url: "http://localhost:3000".to_owned(),
            cors_origins: vec![
                "http://localhost:5173".to_owned(),
                "http://127.0.0.1:5173".to_owned(),
                "http://localhost:4173".to_owned(),
                "http://127.0.0.1:4173".to_owned(),
            ],
            jwt_secret: std::env::var("FLORA_JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-production".to_owned()),
            session_ttl_secs: 3600,
            permission_propagation_grace_secs: 5,
            default_retention_days: 90,
            max_upload_bytes: 100 * 1024 * 1024, // 100 MB
            allowed_file_types: vec![
                "image/*".to_owned(),
                "application/pdf".to_owned(),
                "text/*".to_owned(),
                "video/*".to_owned(),
                "audio/*".to_owned(),
                "application/zip".to_owned(),
            ],
            otel_enabled: false,
            otel_endpoint: None,
        }
    }
}

impl AppConfig {
    /// Returns the validate of this [`AppConfig`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.base_url.is_empty() {
            return Err(crate::Error::Configuration(
                "base_url is required".to_owned(),
            ));
        }
        if self.cors_origins.is_empty() {
            return Err(crate::Error::Configuration(
                "cors_origins must contain at least one origin".to_owned(),
            ));
        }
        for origin in &self.cors_origins {
            if origin.is_empty() {
                return Err(crate::Error::Configuration(
                    "cors_origins contains an empty entry".to_owned(),
                ));
            }
            // tower-http's AllowOrigin::list panics on a literal `*`; fail as a config
            // error instead. Rejecting any `*` substring is deliberately stricter.
            if origin.contains('*') {
                return Err(crate::Error::Configuration(format!(
                    "cors_origins entries must not contain '*': {origin}"
                )));
            }
            // cors_layer() filters unparseable entries; validate first so a typo
            // cannot silently remove an origin from the allowlist.
            if axum::http::HeaderValue::from_str(origin).is_err() {
                return Err(crate::Error::Configuration(format!(
                    "cors_origins entry is not a valid HTTP header value: {origin}"
                )));
            }
            if self.env == "production"
                && (origin.contains("localhost") || origin.contains("127.0.0.1"))
            {
                tracing::warn!(
                    origin = %origin,
                    "cors_origins contains a loopback origin in production"
                );
            }
        }
        if self.jwt_secret.len() < 32 {
            if self.env == "production" {
                return Err(crate::Error::Configuration(
                    "jwt_secret must be at least 32 characters in production".to_owned(),
                ));
            }
            tracing::warn!("Using short JWT secret in production is insecure");
        }
        if self.session_ttl_secs == 0 {
            return Err(crate::Error::Configuration(
                "session_ttl_secs must be > 0".to_owned(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;

    fn config() -> AppConfig {
        AppConfig::default()
    }

    #[test]
    fn valid_default_origins_pass() {
        assert!(config().validate().is_ok());
    }

    #[test]
    fn wildcard_origin_is_rejected() {
        let mut cfg = config();
        cfg.cors_origins = vec!["*".to_owned()];
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn empty_origin_is_rejected() {
        let mut cfg = config();
        cfg.cors_origins = vec![String::new()];
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn control_character_origin_is_rejected() {
        let mut cfg = config();
        // HeaderValue accepts spaces but rejects control characters such as '\n'.
        cfg.cors_origins = vec!["http://bad\nhost".to_owned()];
        assert!(cfg.validate().is_err());
    }
}
