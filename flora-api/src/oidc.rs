//! OIDC ID-token verification (cycle 024, Seed identity).
//!
//! ID tokens obtained from the token endpoint are verified against the issuer's
//! JWKS before any user is created or any Flora session is issued. Claim checks
//! cover signature, `iss`, `aud`, `exp` and `nonce`.

use jsonwebtoken::{
    DecodingKey, Validation, decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
};
use serde::{Deserialize, Serialize};

use flora_core::config::oidc::OidcConfig;
use flora_core::error::{Error, Result};

use crate::state::oidc_transactions::OidcTransaction;

/// Verified identity claims returned after OIDC code exchange and ID-token verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedOidcIdentity {
    /// Stable provider subject (`sub` claim).
    pub sub: String,
    /// Normalized lowercase user e-mail address.
    pub email: String,
    /// Display name from the `name` claim, or `"User"` when absent/blank.
    pub name: String,
}

/// Claims consumed from a verified OIDC ID token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdTokenClaims {
    /// Stable subject identifier at the provider.
    pub sub: String,
    /// Issuer of the token.
    pub iss: String,
    /// Audience of the token.
    pub aud: String,
    /// Expiration time (Unix timestamp).
    pub exp: i64,
    /// One-time nonce bound to the authorization transaction.
    pub nonce: Option<String>,
    /// User email, when provided by the provider.
    #[serde(default)]
    pub email: Option<String>,
    /// Display name, when provided by the provider.
    #[serde(default)]
    pub name: Option<String>,
}

/// Verifies a signed ID token against a JWKS and the expected transaction values.
///
/// `jwks` is passed in so the check is deterministic and testable without a live
/// provider; production callers obtain it with [`fetch_jwks`].
///
/// # Errors
///
/// Returns an OIDC error when the header, signature, issuer, audience, expiry,
/// subject or nonce is missing or invalid.
pub fn verify_id_token(
    token: &str,
    expected_issuer: &str,
    expected_audience: &str,
    expected_nonce: &str,
    jwks: &JwkSet,
) -> Result<IdTokenClaims> {
    verify_id_token_inner(
        token,
        expected_issuer,
        expected_audience,
        expected_nonce,
        jwks,
        false,
    )
}

/// Implementation shared with unit tests, which use an HS256 fixture JWK.
fn verify_id_token_inner(
    token: &str,
    expected_issuer: &str,
    expected_audience: &str,
    expected_nonce: &str,
    jwks: &JwkSet,
    allow_symmetric: bool,
) -> Result<IdTokenClaims> {
    let header = decode_header(token).map_err(|e| Error::OidcProvider(e.to_string()))?;
    let kid = header
        .kid
        .as_deref()
        .ok_or_else(|| Error::OidcProvider("id_token header missing kid".to_owned()))?;
    let jwk = jwks
        .find(kid)
        .ok_or_else(|| Error::OidcProvider("id_token kid not found in JWKS".to_owned()))?;

    // Never accept a symmetric key from a provider JWKS for an ID token.
    if !allow_symmetric && matches!(jwk.algorithm, AlgorithmParameters::OctetKey(_)) {
        return Err(Error::OidcProvider(
            "id_token signed with a symmetric JWK is not accepted".to_owned(),
        ));
    }

    let key = DecodingKey::from_jwk(jwk).map_err(|e| Error::OidcProvider(e.to_string()))?;
    let mut validation = Validation::new(header.alg);
    validation.set_issuer(&[expected_issuer]);
    validation.set_audience(&[expected_audience]);
    validation.required_spec_claims.extend([
        "exp".to_owned(),
        "iss".to_owned(),
        "aud".to_owned(),
        "sub".to_owned(),
    ]);

    let data = decode::<IdTokenClaims>(token, &key, &validation)
        .map_err(|e| Error::OidcProvider(format!("id_token verification failed: {e}")))?;

    let claims = data.claims;
    if claims.nonce.as_deref() != Some(expected_nonce) {
        return Err(Error::OidcProvider("id_token nonce mismatch".to_owned()));
    }
    if claims.sub.trim().is_empty() {
        return Err(Error::OidcProvider("id_token missing sub".to_owned()));
    }
    Ok(claims)
}

/// Fetches provider discovery metadata and the JWKS referenced by it.
///
/// # Errors
///
/// Returns an OIDC error when discovery or the JWKS cannot be fetched/parsed.
pub async fn fetch_jwks(issuer: &str) -> Result<JwkSet> {
    let issuer = issuer.trim_end_matches('/');
    let client = reqwest::Client::new();
    let discovery_url = format!("{issuer}/.well-known/openid-configuration");
    let discovery = client
        .get(&discovery_url)
        .send()
        .await
        .map_err(|e| Error::OidcProvider(format!("OIDC discovery failed: {e}")))?;
    if !discovery.status().is_success() {
        return Err(Error::OidcProvider(format!(
            "OIDC discovery returned {}",
            discovery.status()
        )));
    }
    let metadata: serde_json::Value = discovery
        .json()
        .await
        .map_err(|e| Error::OidcProvider(format!("invalid OIDC discovery document: {e}")))?;
    let jwks_uri = metadata["jwks_uri"]
        .as_str()
        .ok_or_else(|| Error::OidcProvider("OIDC discovery missing jwks_uri".to_owned()))?;
    if !jwks_uri.starts_with("https://")
        && !jwks_uri.starts_with("http://localhost")
        && !jwks_uri.starts_with("http://127.0.0.1")
    {
        return Err(Error::OidcProvider(
            "OIDC jwks_uri must use HTTPS (or localhost for development)".to_owned(),
        ));
    }

    let response = client
        .get(jwks_uri)
        .send()
        .await
        .map_err(|e| Error::OidcProvider(format!("JWKS fetch failed: {e}")))?;
    if !response.status().is_success() {
        return Err(Error::OidcProvider(format!(
            "JWKS fetch returned {}",
            response.status()
        )));
    }
    let jwks: JwkSet = response
        .json()
        .await
        .map_err(|e| Error::OidcProvider(format!("invalid JWKS: {e}")))?;
    Ok(jwks)
}

/// Exchanges an authorization `code` at the provider token endpoint, verifies the
/// returned ID token against Discovery/JWKS, and extracts a normalized identity.
///
/// # Errors
///
/// Returns [`Error::OidcProvider`] if form encoding, token exchange HTTP call,
/// discovery/JWKS fetch, signature/claim verification, or required `email`
/// extraction fails.
pub async fn exchange_code_and_verify_id_token(
    oidc: &OidcConfig,
    code: &str,
    transaction: &OidcTransaction,
) -> Result<VerifiedOidcIdentity> {
    let issuer = oidc.issuer_url.trim_end_matches('/').to_owned();
    let token_url = format!("{issuer}/oauth/v2/token");

    let client = reqwest::Client::new();
    let form_data = serde_urlencoded::to_string([
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", transaction.redirect_uri.as_str()),
        ("client_id", oidc.client_id.as_str()),
        ("client_secret", oidc.client_secret.as_str()),
        ("code_verifier", transaction.code_verifier.as_str()),
    ])
    .map_err(|e| Error::OidcProvider(e.to_string()))?;

    let token_response = client
        .post(&token_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()
        .await
        .map_err(|e| Error::OidcProvider(e.to_string()))?;

    if !token_response.status().is_success() {
        return Err(Error::OidcProvider(format!(
            "token exchange failed: {}",
            token_response.status()
        )));
    }

    let token_body: serde_json::Value = token_response
        .json()
        .await
        .map_err(|e| Error::OidcProvider(e.to_string()))?;

    let id_token = token_body["id_token"]
        .as_str()
        .ok_or_else(|| Error::OidcProvider("missing id_token".to_owned()))?;

    // 024: verify signature + iss/aud/exp/nonce against the provider's JWKS before
    // creating a user or issuing a Flora session.
    let jwks = fetch_jwks(&issuer).await?;
    let claims = verify_id_token(
        id_token,
        &issuer,
        oidc.client_id.as_str(),
        transaction.nonce.as_str(),
        &jwks,
    )?;

    let email = claims
        .email
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_lowercase)
        .ok_or_else(|| Error::OidcProvider("id_token missing email".to_owned()))?;

    let name = claims
        .name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("User")
        .to_owned();

    Ok(VerifiedOidcIdentity {
        sub: claims.sub,
        email,
        name,
    })
}

#[cfg(test)]
#[expect(
    clippy::expect_used,
    reason = "test-only JWKS/fixture construction: a failure here is a test bug"
)]
mod tests {
    use super::*;
    use base64::Engine;
    use chrono::{Duration, Utc};
    use jsonwebtoken::{EncodingKey, Header, encode};
    use serde_json::json;

    const SECRET: &[u8] = b"flora-oidc-test-secret-0123456789abcdef";
    const KID: &str = "test-key";

    fn jwks() -> JwkSet {
        let k = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(SECRET);
        serde_json::from_value(json!({
            "keys": [{ "kty": "oct", "alg": "HS256", "kid": KID, "k": k }]
        }))
        .expect("valid test JWKS")
    }

    fn token(
        alg: jsonwebtoken::Algorithm,
        kid: &str,
        issuer: &str,
        audience: &str,
        nonce: Option<&str>,
        subject: &str,
    ) -> String {
        let claims = json!({
            "sub": subject,
            "iss": issuer,
            "aud": audience,
            "exp": (Utc::now() + Duration::hours(1)).timestamp(),
            "nonce": nonce,
            "email": "user@example.test",
            "name": "Test User",
        });
        let mut header = Header::new(alg);
        header.kid = Some(kid.to_owned());
        encode(&header, &claims, &EncodingKey::from_secret(SECRET)).expect("valid test token")
    }

    #[test]
    fn verifies_valid_token() {
        let token = token(
            jsonwebtoken::Algorithm::HS256,
            KID,
            "https://issuer.test",
            "flora-client",
            Some("nonce-1"),
            "subject-1",
        );
        let claims = verify_id_token_inner(
            &token,
            "https://issuer.test",
            "flora-client",
            "nonce-1",
            &jwks(),
            true,
        )
        .expect("valid token");
        assert_eq!(claims.sub, "subject-1");
        assert_eq!(claims.email.as_deref(), Some("user@example.test"));
    }

    #[test]
    fn rejects_tampered_token() {
        let mut token = token(
            jsonwebtoken::Algorithm::HS256,
            KID,
            "https://issuer.test",
            "flora-client",
            Some("nonce-1"),
            "subject-1",
        );
        token.push('x');
        assert!(
            verify_id_token_inner(
                &token,
                "https://issuer.test",
                "flora-client",
                "nonce-1",
                &jwks(),
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_wrong_issuer_audience_or_nonce() {
        let token = token(
            jsonwebtoken::Algorithm::HS256,
            KID,
            "https://issuer.test",
            "flora-client",
            Some("nonce-1"),
            "subject-1",
        );
        assert!(
            verify_id_token_inner(
                &token,
                "https://evil.test",
                "flora-client",
                "nonce-1",
                &jwks(),
                true,
            )
            .is_err()
        );
        assert!(
            verify_id_token_inner(
                &token,
                "https://issuer.test",
                "other-client",
                "nonce-1",
                &jwks(),
                true,
            )
            .is_err()
        );
        assert!(
            verify_id_token_inner(
                &token,
                "https://issuer.test",
                "flora-client",
                "nonce-2",
                &jwks(),
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_unknown_kid() {
        let token = token(
            jsonwebtoken::Algorithm::HS256,
            "unknown",
            "https://issuer.test",
            "flora-client",
            Some("nonce-1"),
            "subject-1",
        );
        assert!(
            verify_id_token_inner(
                &token,
                "https://issuer.test",
                "flora-client",
                "nonce-1",
                &jwks(),
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn jwks_find_is_kid_aware() {
        let set = jwks();
        assert!(set.find(KID).is_some());
        assert!(set.find("missing").is_none());
    }

    #[test]
    fn verify_id_token_accepts_asymmetric_eddsa_and_rejects_symmetric_jwk() {
        let ed_der: [u8; 48] = [
            0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22,
            0x04, 0x20, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
        ];
        let ed_key = EncodingKey::from_ed_der(&ed_der);
        let mut ed_jwk =
            jsonwebtoken::jwk::Jwk::from_encoding_key(&ed_key, jsonwebtoken::Algorithm::EdDSA)
                .expect("valid Ed25519 JWK");
        ed_jwk.common.key_id = Some("ed-1".to_owned());
        let ed_jwks = JwkSet { keys: vec![ed_jwk] };

        let claims = json!({
            "sub": "ed-sub-1",
            "iss": "https://issuer.test",
            "aud": "flora-client",
            "exp": (Utc::now() + Duration::hours(1)).timestamp(),
            "nonce": "nonce-ed",
            "email": "ed@example.test",
            "name": "Ed User",
        });
        let mut ed_header = Header::new(jsonwebtoken::Algorithm::EdDSA);
        ed_header.kid = Some("ed-1".to_owned());
        let ed_token = encode(&ed_header, &claims, &ed_key).expect("signed EdDSA token");

        let verified = verify_id_token(
            &ed_token,
            "https://issuer.test",
            "flora-client",
            "nonce-ed",
            &ed_jwks,
        )
        .expect("public verify_id_token must accept asymmetric EdDSA JWK");
        assert_eq!(verified.sub, "ed-sub-1");

        let sym_token = token(
            jsonwebtoken::Algorithm::HS256,
            KID,
            "https://issuer.test",
            "flora-client",
            Some("nonce-sym"),
            "sym-sub-1",
        );
        let err = verify_id_token(
            &sym_token,
            "https://issuer.test",
            "flora-client",
            "nonce-sym",
            &jwks(),
        )
        .expect_err("public verify_id_token must reject symmetric JWK");
        assert!(err.to_string().contains("symmetric JWK"));
    }

    #[tokio::test]
    async fn fetch_jwks_rejects_non_localhost_http_jwks_uri() {
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path(
                "/.well-known/openid-configuration",
            ))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(json!({
                "issuer": server.uri(),
                "jwks_uri": "http://evil.example.com/keys",
            })))
            .mount(&server)
            .await;

        let err = fetch_jwks(&server.uri())
            .await
            .expect_err("non-localhost http jwks_uri must be rejected");
        assert!(err.to_string().contains("must use HTTPS"));
    }
}
