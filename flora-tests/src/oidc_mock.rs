//! Ephemeral local HTTP OIDC Identity Provider mock for integration and contract tests.
//!
//! Serves OpenID Connect Discovery (`/.well-known/openid-configuration`),
//! JWKS (`/oauth/v2/keys` with asymmetric `Ed25519` `OKP` keys by default), and
//! OAuth 2.0 / PKCE S256 token exchange (`POST /oauth/v2/token`).

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, EncodingKey, Header, encode,
    jwk::{Jwk, JwkSet, PublicKeyUse},
};
use serde::Serialize;
use serde_json::json;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Default asymmetric key identifier published by [`MockOidcIdpServer`].
pub const MOCK_IDP_KID: &str = "mock-idp-ed25519-1";

/// Shared symmetric secret used only when simulating a hostile/misconfigured `IdP` JWKS.
const MOCK_SYM_SECRET: &[u8] = b"mock-idp-symmetric-secret-0123456789";

/// Deterministic PKCS#8 v1 Ed25519 DER private key (16-byte header + 32-byte seed).
const ED25519_PKCS8_V1_DER: [u8; 48] = [
    0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22, 0x04, 0x20,
    0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
    0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
];

/// Configuration for a single-use authorization code issued by [`MockOidcIdpServer`].
#[derive(Debug, Clone)]
pub struct MockAuthorizationGrant {
    /// Subject (`sub`) claim embedded in the signed ID token.
    pub sub: String,
    /// Optional email claim (`email`) embedded in the signed ID token.
    pub email: Option<String>,
    /// Optional `email_verified` claim embedded in the signed ID token (defaults to `Some(true)`).
    pub email_verified: Option<bool>,
    /// Optional display name claim (`name`) embedded in the signed ID token.
    pub name: Option<String>,
    /// Expected `redirect_uri` matched on `POST /oauth/v2/token`.
    pub redirect_uri: String,
    /// Expected PKCE S256 `code_challenge` validated against `code_verifier`.
    pub code_challenge: String,
    /// Nonce bound to the login transaction and embedded in the ID token.
    pub nonce: String,
    /// If `true`, alters a byte inside the Ed25519 signature segment to simulate tampering.
    pub tamper_signature: bool,
    /// If `true`, returns a token response without `id_token`.
    pub omit_id_token: bool,
    /// Optional token expiration offset in seconds (defaults to +3600).
    pub exp_offset_secs: i64,
}

impl MockAuthorizationGrant {
    /// Constructs a valid default authorization grant bound to `redirect_uri`, `code_challenge`, and `nonce`.
    #[must_use]
    pub fn new(
        sub: impl Into<String>,
        email: Option<String>,
        name: Option<String>,
        redirect_uri: impl Into<String>,
        code_challenge: impl Into<String>,
        nonce: impl Into<String>,
    ) -> Self {
        Self {
            sub: sub.into(),
            email,
            email_verified: Some(true),
            name,
            redirect_uri: redirect_uri.into(),
            code_challenge: code_challenge.into(),
            nonce: nonce.into(),
            tamper_signature: false,
            omit_id_token: false,
            exp_offset_secs: 3600,
        }
    }
}

/// Recorded form payload received by `POST /oauth/v2/token`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedTokenRequest {
    /// Received `grant_type` parameter.
    pub grant_type: Option<String>,
    /// Received `code` parameter.
    pub code: Option<String>,
    /// Received `redirect_uri` parameter.
    pub redirect_uri: Option<String>,
    /// Received `client_id` parameter.
    pub client_id: Option<String>,
    /// Received `client_secret` parameter.
    pub client_secret: Option<String>,
    /// Received `code_verifier` parameter.
    pub code_verifier: Option<String>,
}

#[derive(Debug)]
struct MockIdpState {
    issuer_url: String,
    client_id: String,
    client_secret: String,
    ed_encoding_key: EncodingKey,
    ed_jwks: JwkSet,
    sym_jwks: JwkSet,
    use_symmetric_jwks: bool,
    jwks_uri_override: Option<String>,
    grants: HashMap<String, MockAuthorizationGrant>,
    token_requests: Vec<ObservedTokenRequest>,
}

#[derive(Debug, Serialize)]
struct MockIdTokenClaims {
    sub: String,
    iss: String,
    aud: String,
    exp: i64,
    nonce: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

/// Ephemeral HTTP OIDC Identity Provider bound to `127.0.0.1:0`.
#[derive(Debug)]
pub struct MockOidcIdpServer {
    issuer_url: String,
    state: Arc<Mutex<MockIdpState>>,
    handle: JoinHandle<()>,
}

impl Drop for MockOidcIdpServer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl MockOidcIdpServer {
    /// Starts an isolated mock OIDC HTTP server bound to `127.0.0.1:0`.
    ///
    /// # Errors
    ///
    /// Returns an error if the Ed25519 JWK cannot be built or the local TCP listener cannot bind.
    pub async fn start(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> anyhow::Result<Self> {
        let ed_encoding_key = EncodingKey::from_ed_der(&ED25519_PKCS8_V1_DER);
        let mut ed_jwk = Jwk::from_encoding_key(&ed_encoding_key, Algorithm::EdDSA)?;
        ed_jwk.common.key_id = Some(MOCK_IDP_KID.to_owned());
        ed_jwk.common.public_key_use = Some(PublicKeyUse::Signature);
        let ed_jwks = JwkSet { keys: vec![ed_jwk] };

        let sym_encoding_key = EncodingKey::from_secret(MOCK_SYM_SECRET);
        let mut sym_jwk = Jwk::from_encoding_key(&sym_encoding_key, Algorithm::HS256)?;
        sym_jwk.common.key_id = Some(MOCK_IDP_KID.to_owned());
        sym_jwk.common.public_key_use = Some(PublicKeyUse::Signature);
        let sym_jwks = JwkSet {
            keys: vec![sym_jwk],
        };

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let issuer_url = format!("http://{addr}");

        let state = Arc::new(Mutex::new(MockIdpState {
            issuer_url: issuer_url.clone(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            ed_encoding_key,
            ed_jwks,
            sym_jwks,
            use_symmetric_jwks: false,
            jwks_uri_override: None,
            grants: HashMap::new(),
            token_requests: Vec::new(),
        }));

        let router = Router::new()
            .route("/.well-known/openid-configuration", get(discovery))
            .route("/oauth/v2/keys", get(jwks))
            .route("/oauth/v2/token", post(token))
            .with_state(Arc::clone(&state));

        let handle = tokio::spawn(async move {
            let _ = axum::serve(listener, router).await;
        });

        Ok(Self {
            issuer_url,
            state,
            handle,
        })
    }

    /// Base issuer URL (`http://127.0.0.1:<port>`) matching discovery and `iss` claims.
    #[must_use]
    pub fn issuer_url(&self) -> &str {
        &self.issuer_url
    }

    /// Registers a single-use authorization code to be accepted by `POST /oauth/v2/token`.
    pub async fn register_code(&self, code: impl Into<String>, grant: MockAuthorizationGrant) {
        let mut guard = self.state.lock().await;
        let _ = guard.grants.insert(code.into(), grant);
    }

    /// Toggles returning a symmetric HS256 `oct` JWK (and signing with HS256) to verify symmetric rejection.
    pub async fn set_use_symmetric_jwks(&self, enabled: bool) {
        let mut guard = self.state.lock().await;
        guard.use_symmetric_jwks = enabled;
    }

    /// Overrides the `jwks_uri` advertised in `/.well-known/openid-configuration`.
    pub async fn set_jwks_uri_override(&self, uri: Option<String>) {
        let mut guard = self.state.lock().await;
        guard.jwks_uri_override = uri;
    }

    /// Drains and returns all form payloads observed by `POST /oauth/v2/token`.
    pub async fn take_token_requests(&self) -> Vec<ObservedTokenRequest> {
        let mut guard = self.state.lock().await;
        std::mem::take(&mut guard.token_requests)
    }
}

async fn discovery(State(state): State<Arc<Mutex<MockIdpState>>>) -> impl IntoResponse {
    let (issuer, jwks_uri) = {
        let guard = state.lock().await;
        let issuer = guard.issuer_url.clone();
        let jwks_uri = guard
            .jwks_uri_override
            .clone()
            .unwrap_or_else(|| format!("{issuer}/oauth/v2/keys"));
        drop(guard);
        (issuer, jwks_uri)
    };
    Json(json!({
        "issuer": issuer,
        "authorization_endpoint": format!("{issuer}/oauth/v2/authorize"),
        "token_endpoint": format!("{issuer}/oauth/v2/token"),
        "jwks_uri": jwks_uri,
    }))
}

async fn jwks(State(state): State<Arc<Mutex<MockIdpState>>>) -> impl IntoResponse {
    let payload = {
        let guard = state.lock().await;
        if guard.use_symmetric_jwks {
            guard.sym_jwks.clone()
        } else {
            guard.ed_jwks.clone()
        }
    };
    Json(payload)
}

fn parse_token_form(body: &str) -> Result<ObservedTokenRequest, String> {
    let parsed: Vec<(String, String)> =
        serde_urlencoded::from_str(body).map_err(|err| err.to_string())?;
    let find = |key: &str| {
        parsed
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    };
    Ok(ObservedTokenRequest {
        grant_type: find("grant_type"),
        code: find("code"),
        redirect_uri: find("redirect_uri"),
        client_id: find("client_id"),
        client_secret: find("client_secret"),
        code_verifier: find("code_verifier"),
    })
}

fn consume_valid_grant(
    guard: &mut MockIdpState,
    observed: &ObservedTokenRequest,
) -> Result<MockAuthorizationGrant, &'static str> {
    if observed.grant_type.as_deref() != Some("authorization_code") {
        return Err("unsupported grant_type");
    }
    if observed.client_id.as_deref() != Some(guard.client_id.as_str())
        || observed.client_secret.as_deref() != Some(guard.client_secret.as_str())
    {
        return Err("invalid client credentials");
    }
    let Some(code) = observed.code.as_deref() else {
        return Err("missing authorization code");
    };
    let Some(grant) = guard.grants.remove(code) else {
        return Err("unknown or already used authorization code");
    };
    if observed.redirect_uri.as_deref() != Some(grant.redirect_uri.as_str()) {
        return Err("redirect_uri mismatch");
    }
    let Some(verifier) = observed.code_verifier.as_deref() else {
        return Err("missing code_verifier");
    };
    if flora_core::utils::pkce::derive_s256_challenge(verifier) != grant.code_challenge {
        return Err("pkce code_verifier mismatch");
    }
    Ok(grant)
}

fn sign_mock_id_token(
    grant: &MockAuthorizationGrant,
    issuer_url: String,
    client_id: String,
    ed_encoding_key: &EncodingKey,
    use_symmetric_jwks: bool,
) -> Option<String> {
    let claims = MockIdTokenClaims {
        sub: grant.sub.clone(),
        iss: issuer_url,
        aud: client_id,
        exp: (Utc::now() + Duration::seconds(grant.exp_offset_secs)).timestamp(),
        nonce: grant.nonce.clone(),
        email: grant.email.clone(),
        email_verified: grant.email_verified,
        name: grant.name.clone(),
    };
    let (alg, key) = if use_symmetric_jwks {
        (Algorithm::HS256, EncodingKey::from_secret(MOCK_SYM_SECRET))
    } else {
        (Algorithm::EdDSA, ed_encoding_key.clone())
    };
    let mut header = Header::new(alg);
    header.kid = Some(MOCK_IDP_KID.to_owned());

    let mut id_token = encode(&header, &claims, &key).ok()?;
    if grant.tamper_signature && id_token.len() > 20 {
        let mut bytes = id_token.into_bytes();
        let idx = bytes.len().saturating_sub(10);
        bytes[idx] = if bytes[idx] == b'a' { b'b' } else { b'a' };
        id_token = String::from_utf8(bytes).ok()?;
    }
    Some(id_token)
}

async fn token(
    State(state): State<Arc<Mutex<MockIdpState>>>,
    body: String,
) -> axum::response::Response {
    let observed = match parse_token_form(&body) {
        Ok(observed) => observed,
        Err(description) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "invalid_request", "error_description": description })),
            )
                .into_response();
        }
    };

    let (issuer_url, client_id, ed_encoding_key, use_symmetric_jwks, grant_result) = {
        let mut guard = state.lock().await;
        guard.token_requests.push(observed.clone());
        let result = consume_valid_grant(&mut guard, &observed);
        let issuer = guard.issuer_url.clone();
        let cid = guard.client_id.clone();
        let ed_key = guard.ed_encoding_key.clone();
        let use_sym = guard.use_symmetric_jwks;
        drop(guard);
        (issuer, cid, ed_key, use_sym, result)
    };

    let grant = match grant_result {
        Ok(grant) => grant,
        Err(reason) => return invalid_grant(reason),
    };
    if grant.omit_id_token {
        return (
            StatusCode::OK,
            Json(json!({
                "access_token": "mock-access-token",
                "token_type": "Bearer",
                "expires_in": 3600,
            })),
        )
            .into_response();
    }
    let Some(id_token) = sign_mock_id_token(
        &grant,
        issuer_url,
        client_id,
        &ed_encoding_key,
        use_symmetric_jwks,
    ) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "server_error" })),
        )
            .into_response();
    };

    (
        StatusCode::OK,
        Json(json!({
            "access_token": "mock-access-token",
            "token_type": "Bearer",
            "expires_in": 3600,
            "id_token": id_token,
        })),
    )
        .into_response()
}

fn invalid_grant(description: &str) -> axum::response::Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": "invalid_grant",
            "error_description": description,
        })),
    )
        .into_response()
}
