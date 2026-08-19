//! # Flora Core JWT Utilities
//!
//! JSON Web Token utilities for Flora session management.
//!
//! Flora issues its own session JWTs after OIDC authentication.
//! These tokens are signed with HMAC-SHA256 and scoped to a single organization.
#![allow(
    clippy::expect_used,
    reason = "JWT encoding must not fail with valid claims and a non-empty secret"
)]
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// JWT claims for a Flora session token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloraClaims {
    /// Subject: user ID.
    pub sub: String,
    /// Session JTI (unique token ID).
    pub jti: String,
    /// The organization this session is scoped to.
    pub organization_id: String,
    /// User email for display.
    pub email: String,
    /// Expiration time (Unix timestamp).
    pub exp: i64,
    /// Issued-at time (Unix timestamp).
    pub iat: i64,
    /// Token version (for invalidation).
    #[serde(default)]
    pub ver: u32,
}

impl FloraClaims {
    /// Creates new claims for a session.
    #[must_use]
    pub fn new(
        user_id: uuid::Uuid,
        jti: String,
        organization_id: uuid::Uuid,
        email: &str,
        ttl_secs: u64,
    ) -> Self {
        let now = Utc::now();
        let exp = now + Duration::seconds(i64::try_from(ttl_secs).unwrap_or(36_000));
        Self {
            sub: user_id.to_string(),
            jti,
            organization_id: organization_id.to_string(),
            email: email.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            ver: 1,
        }
    }

    /// Returns the user ID parsed from the `sub` claim.
    #[must_use]
    pub fn user_id(&self) -> uuid::Uuid {
        uuid::Uuid::parse_str(&self.sub).unwrap_or_default()
    }

    /// Returns the organization ID parsed from the `organization_id` claim.
    #[must_use]
    pub fn org_id(&self) -> uuid::Uuid {
        uuid::Uuid::parse_str(&self.organization_id).unwrap_or_default()
    }
}

/// Encodes a Flora JWT session token.
///
/// # Panics
///
/// Panics only if encoding fails with a valid secret (should not happen).
#[must_use]
#[allow(
    clippy::allow_attributes,
    clippy::expect_used,
    reason = "JWT encoding must not fail with valid claims and a non-empty secret"
)]
pub fn encode_token(claims: &FloraClaims, secret: &str) -> String {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("JWT encoding must not fail with valid claims and a non-empty secret")
}

/// Decodes and validates a Flora JWT session token.
///
/// # Errors
///
/// Returns an error if the token is malformed, expired, or has an invalid signature.
pub fn decode_token(token: &str, secret: &str) -> Result<FloraClaims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<FloraClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_and_decode_roundtrip() {
        let claims = FloraClaims::new(
            uuid::Uuid::now_v7(),
            "jti-test-123".to_string(),
            uuid::Uuid::now_v7(),
            "alice@example.com",
            3600,
        );
        let secret = "test-secret-key-at-least-32-chars-long!!";
        let token = encode_token(&claims, secret);
        #[allow(
            clippy::allow_attributes,
            clippy::expect_used,
            reason = "Test assertion: token should decode cleanly in roundtrip test"
        )]
        let decoded = decode_token(&token, secret).expect("should decode cleanly");
        assert_eq!(decoded.sub, claims.sub);
        assert_eq!(decoded.email, "alice@example.com");
        assert_eq!(decoded.organization_id, claims.organization_id);
    }

    #[test]
    fn decode_rejects_tampered_token() {
        let claims = FloraClaims::new(
            uuid::Uuid::now_v7(),
            "jti-test-456".to_string(),
            uuid::Uuid::now_v7(),
            "bob@example.com",
            3600,
        );
        let secret = "test-secret-key-at-least-32-chars-long!!";
        let token = encode_token(&claims, secret);
        let tampered = format!("{token}x");
        assert!(decode_token(&tampered, secret).is_err());
    }

    #[test]
    fn decode_rejects_wrong_secret() {
        let claims = FloraClaims::new(
            uuid::Uuid::now_v7(),
            "jti-test-789".to_string(),
            uuid::Uuid::now_v7(),
            "carol@example.com",
            3600,
        );
        let token = encode_token(&claims, "correct-secret-32-characters-long!!");
        let wrong = decode_token(&token, "wrong-secret-32-characters-long!!");
        assert!(wrong.is_err());
    }
}
