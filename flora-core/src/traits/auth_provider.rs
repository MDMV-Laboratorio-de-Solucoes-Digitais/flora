//! Authentication provider trait.
//!
//! Abstracts the OIDC provider (Zitadel) to allow for mocking and fallback mechanisms.

use async_trait::async_trait;

use crate::error::Result;

/// Authentication provider trait for OIDC integration.
///
/// This abstraction allows swapping between different OIDC providers
/// or implementing fallback mechanisms (e.g., local email/password).
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Initiates the OIDC login flow.
    ///
    /// Returns the authorization URL to redirect the user to.
    async fn initiate_login(&self, redirect_uri: &str) -> Result<String>;

    /// Handles the OIDC callback and exchanges the code for tokens.
    ///
    /// Returns the user information extracted from the ID token.
    async fn handle_callback(&self, code: &str, redirect_uri: &str) -> Result<UserInfo>;

    /// Validates an access token and returns the user information.
    async fn validate_token(&self, access_token: &str) -> Result<UserInfo>;

    /// Refreshes an access token using a refresh token.
    async fn refresh_token(&self, refresh_token: &str) -> Result<String>;

    /// Logs out the user (revokes tokens with the OIDC provider).
    async fn logout(&self, access_token: &str) -> Result<()>;

    /// Checks if the OIDC provider is available.
    async fn is_available(&self) -> bool;

    /// Returns the issuer URL for the OIDC provider.
    fn issuer_url(&self) -> &str;

    /// Returns the client ID for the OIDC provider.
    fn client_id(&self) -> &str;
}

/// User information extracted from OIDC tokens.
#[derive(Debug, Clone)]
pub struct UserInfo {
    /// Subject identifier (unique user ID).
    pub sub: String,
    /// User email address.
    pub email: String,
    /// Whether the email has been verified.
    pub email_verified: bool,
    /// User display name.
    pub name: String,
    /// User given name (first name).
    pub given_name: Option<String>,
    /// User family name (last name).
    pub family_name: Option<String>,
    /// User profile picture URL.
    pub picture: Option<String>,
    /// User locale/language preference.
    pub locale: Option<String>,
    /// Timestamp of last profile update.
    pub updated_at: Option<u64>,
}
