//! Authentication service stub implementing the `AuthProvider` trait.
//!
//! The real OIDC integration (Zitadel) will be added in a subsequent iteration.

use async_trait::async_trait;

use crate::error::{Error, Result};
use crate::traits::auth_provider::{AuthProvider, UserInfo};

/// Stub implementation of `AuthProvider`.
///
/// Methods currently return `Err(Error::ServiceUnavailable(...))` and are intended to be
/// replaced with real OIDC logic in a future iteration.
#[derive(Debug)]
pub struct AuthService {}

impl AuthService {
    /// Creates a new `AuthService` stub.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for AuthService {
    /// Creates a new `AuthService` stub via `Default`.
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthProvider for AuthService {
    async fn initiate_login(&self, _redirect_uri: &str) -> Result<String> {
        Err(Error::ServiceUnavailable(
            "initiate_login not implemented".to_string(),
        ))
    }

    async fn handle_callback(&self, _code: &str, _redirect_uri: &str) -> Result<UserInfo> {
        Err(Error::ServiceUnavailable(
            "handle_callback not implemented".to_string(),
        ))
    }

    async fn validate_token(&self, _access_token: &str) -> Result<UserInfo> {
        Err(Error::ServiceUnavailable(
            "validate_token not implemented".to_string(),
        ))
    }

    async fn refresh_token(&self, _refresh_token: &str) -> Result<String> {
        Err(Error::ServiceUnavailable(
            "refresh_token not implemented".to_string(),
        ))
    }

    async fn logout(&self, _access_token: &str) -> Result<()> {
        Err(Error::ServiceUnavailable(
            "logout not implemented".to_string(),
        ))
    }

    async fn is_available(&self) -> bool {
        false
    }

    fn issuer_url(&self) -> &'static str {
        ""
    }

    fn client_id(&self) -> &'static str {
        ""
    }
}
