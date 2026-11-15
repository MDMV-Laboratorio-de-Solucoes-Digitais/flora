//! PKCE (Proof Key for Code Exchange) utilities for OIDC authorization flows.
//!
//! Implements RFC 7636 S256 challenges for public/confidential clients.

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::Rng;
use sha2::{Digest, Sha256};

/// Length in bytes of random material used for verifier and state values.
/// 32 bytes encodes to 43 URL-safe base64 characters without padding.
const RANDOM_BYTES: usize = 32;

/// Generates a cryptographically random PKCE code verifier.
///
/// Returns a URL-safe base64 string without padding (43 characters).
#[must_use]
pub fn generate_code_verifier() -> String {
    let mut bytes = [0_u8; RANDOM_BYTES];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Derives an S256 code challenge from a PKCE code verifier.
///
/// Returns `BASE64URL(SHA256(verifier))` without padding.
#[must_use]
pub fn derive_s256_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}

/// Generates a cryptographically random OAuth state parameter.
///
/// Returns a URL-safe base64 string without padding.
#[must_use]
pub fn generate_state() -> String {
    let mut bytes = [0_u8; RANDOM_BYTES];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifier_and_state_are_url_safe_and_non_empty() {
        let verifier = generate_code_verifier();
        let state = generate_state();
        assert_ne!(verifier.len(), 0);
        assert_ne!(state.len(), 0);
        assert!(!verifier.contains('='));
        assert!(!state.contains('='));
        assert!(
            verifier
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        );
    }

    #[test]
    fn challenge_is_deterministic_for_verifier() {
        let verifier = "test-verifier";
        let c1 = derive_s256_challenge(verifier);
        let c2 = derive_s256_challenge(verifier);
        assert_eq!(c1, c2);
        assert_ne!(c1.len(), 0);
        assert_ne!(c1, verifier);
    }

    #[test]
    fn different_verifiers_produce_different_challenges() {
        let a = derive_s256_challenge(&generate_code_verifier());
        let b = derive_s256_challenge(&generate_code_verifier());
        assert_ne!(a, b);
    }
}
