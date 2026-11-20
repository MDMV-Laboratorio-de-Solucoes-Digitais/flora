//! Authentication routes — OIDC login, logout, token refresh, and callback.
//!
//! Per T019, T023, and T116 (PKCE).

use axum::{
    Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::models::Session;
use flora_core::repositories::{PgMembershipRepository, PgSessionRepository, PgUserRepository};
use flora_core::traits::{MembershipRepository, SessionRepository, UserRepository};
use flora_core::utils::jwt::{FloraClaims, encode_token};
use flora_core::utils::pkce::{derive_s256_challenge, generate_code_verifier, generate_state};

use crate::extractors::AuthExtractor;
use crate::state::oidc_transactions::OidcTransaction;

use super::AppState;

/// Query parameters for auth routes.
#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    /// The redirect URI for the OIDC callback.
    pub redirect_uri: Option<String>,
}

/// Request body for token refresh.
#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    /// The session token to refresh.
    pub session_token: String,
}

/// Request body for selecting an organization context.
#[derive(Debug, Deserialize)]
pub struct SelectOrganizationRequest {
    /// The organization to scope the session to.
    pub organization_id: Uuid,
}

/// Response body for a successful login.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    /// The new session token.
    pub session_token: String,
    /// Token expiration time in seconds.
    pub expires_in: u64,
    /// The authenticated user's ID.
    pub user_id: String,
}

/// Response body for errors.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// The error type.
    pub error: String,
    /// A human-readable error message.
    pub message: String,
}

/// Creates the `/auth` sub-router.
pub fn create_auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/select-organization", post(select_organization))
}

/// `GET /auth/login` — Returns the OIDC authorization URL for redirect.
///
/// Generates a PKCE verifier/challenge and state, persists the transaction
/// server-side, and returns an authorize URL including `state`,
/// `code_challenge`, and `code_challenge_method`.
async fn login(
    State(state): State<AppState>,
    Query(params): Query<AuthQuery>,
) -> Result<Json<serde_json::Value>> {
    if !state
        .config
        .oidc
        .code_challenge_method
        .eq_ignore_ascii_case("S256")
    {
        return Err(Error::Configuration(
            "oidc.code_challenge_method must be S256".to_owned(),
        ));
    }

    let issuer = state.config.oidc.issuer_url.trim_end_matches('/');
    let client_id = &state.config.oidc.client_id;
    let default_redirect = format!("{}/api/v1/auth/callback", state.config.app.base_url);
    let redirect_uri = params
        .redirect_uri
        .unwrap_or_else(|| default_redirect.clone());
    // 024: exact allowlist — the configured callback plus explicitly allowed URIs.
    if redirect_uri != default_redirect
        && !state
            .config
            .oidc
            .allowed_redirect_uris
            .iter()
            .any(|allowed| allowed == &redirect_uri)
    {
        return Err(Error::OidcProvider(
            "redirect_uri is not allowed".to_owned(),
        ));
    }

    let oauth_state = generate_state();
    let code_verifier = generate_code_verifier();
    let code_challenge = derive_s256_challenge(&code_verifier);
    let nonce = generate_state();

    state
        .oidc_transactions
        .store(
            &oauth_state,
            OidcTransaction {
                code_verifier,
                redirect_uri: redirect_uri.clone(),
                nonce: nonce.clone(),
            },
        )
        .await?;

    let scope = state.config.oidc.scopes.join(" ");
    let mut auth_url = Url::parse(&format!("{issuer}/oauth/v2/authorize"))
        .map_err(|e| Error::Configuration(format!("invalid oidc issuer_url: {e}")))?;
    let _ = auth_url
        .query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("response_type", "code")
        .append_pair("scope", &scope)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("state", &oauth_state)
        .append_pair("code_challenge", &code_challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("nonce", &nonce);

    tracing::info!(redirect_uri = %redirect_uri, "OIDC login initiated with PKCE");

    Ok(Json(serde_json::json!({
        "authorization_url": auth_url.as_str(),
        "state": oauth_state,
        "next_step": "redirect_user_to_authorization_url"
    })))
}

/// `GET /auth/callback` — Handles the OIDC callback from Zitadel.
///
/// Validates and consumes the stored PKCE/state transaction, exchanges the
/// authorization code for tokens with `code_verifier`, creates or finds the
/// user, and issues a Flora session JWT.
#[expect(
    clippy::too_many_lines,
    reason = "OIDC callback: token exchange, ID-token verification and session creation are kept in one transactional flow"
)]
async fn callback(
    State(state): State<AppState>,
    Query(params): Query<OidcCallbackParams>,
) -> Result<Json<serde_json::Value>> {
    let code = params.code.as_deref().ok_or(Error::Unauthorized)?;
    let oauth_state = params.state.as_deref().ok_or(Error::Unauthorized)?;

    let transaction = state
        .oidc_transactions
        .consume(oauth_state)
        .await?
        .ok_or(Error::Unauthorized)?;

    // Exchange code for tokens via Zitadel token endpoint.
    let token_url = format!(
        "{}/oauth/v2/token",
        state.config.oidc.issuer_url.trim_end_matches('/')
    );

    let client = reqwest::Client::new();
    let form_data = serde_urlencoded::to_string([
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", transaction.redirect_uri.as_str()),
        ("client_id", state.config.oidc.client_id.as_str()),
        ("client_secret", state.config.oidc.client_secret.as_str()),
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
        .ok_or_else(|| Error::OidcProvider("missing id_token".to_string()))?;

    // 024: verify signature + iss/aud/exp/nonce against the provider's JWKS before
    // creating a user or issuing a Flora session.
    let issuer = state
        .config
        .oidc
        .issuer_url
        .trim_end_matches('/')
        .to_owned();
    let jwks = crate::oidc::fetch_jwks(&issuer).await?;
    let claims = crate::oidc::verify_id_token(
        id_token,
        &issuer,
        state.config.oidc.client_id.as_str(),
        transaction.nonce.as_str(),
        &jwks,
    )?;
    let sub = claims.sub.clone();
    let email = claims.email.as_deref().unwrap_or("");
    let name = claims.name.as_deref().unwrap_or("User");
    if email.is_empty() {
        return Err(Error::OidcProvider("id_token missing email".to_owned()));
    }

    // Create or update user in the database. Identity is the provider subject (024).
    let user_repo = PgUserRepository::new((*state.db_pool).clone());
    let user = if let Some(existing) = user_repo.find_by_oidc_subject(&sub).await? {
        tracing::info!(user_id = %existing.id, oidc_subject = %sub, "Existing user logged in via OIDC");
        existing
    } else if let Some(existing) = user_repo.find_by_email(email).await? {
        tracing::info!(user_id = %existing.id, email = %email, "Existing user matched by email via OIDC");
        existing
    } else {
        let mut new_user = flora_core::models::User::new(email, name);
        new_user.oidc_subject = Some(sub.clone());
        let created_user = user_repo.create(new_user).await?;
        tracing::info!(user_id = %created_user.id, email = %email, oidc_subject = %sub, "New user registered via OIDC");
        created_user
    };

    // Create a session record.
    let session_repo = PgSessionRepository::new((*state.db_pool).clone());
    let jti = Uuid::now_v7().to_string();
    let ttl = state.config.app.session_ttl_secs;

    // For callback, we use a placeholder org until the user picks one.
    let org_id = Uuid::nil();

    let now = chrono::Utc::now();
    let expires_at = now + chrono::Duration::seconds(ttl.cast_signed());
    let session = Session {
        id: Uuid::now_v7(),
        user_id: user.id,
        organization_id: org_id,
        jti: jti.clone(),
        refresh_token_id: None,
        client_ip: None,
        user_agent: None,
        expires_at,
        last_activity_at: now,
        is_active: true,
        created_at: now,
        updated_at: now,
    };
    let _session = session_repo.create(session).await?;

    let flora_claims = FloraClaims::new(user.id, jti, org_id, email, ttl);
    let jwt = encode_token(&flora_claims, &state.config.app.jwt_secret);

    Ok(Json(serde_json::json!({
        "session_token": jwt,
        "expires_in": ttl,
        "user_id": user.id.to_string(),
        "next_step": "select_or_create_organization"
    })))
}

/// `POST /auth/logout` — Revokes the session and invalidates the token.
async fn logout(State(state): State<AppState>, headers: HeaderMap) -> Result<StatusCode> {
    if let Some(auth_header) = headers.get("authorization")
        && let Ok(token) = auth_header.to_str()
        && let Some(jwt) = token.strip_prefix("Bearer ")
        && let Ok(claims) = flora_core::utils::jwt::decode_token(jwt, &state.config.app.jwt_secret)
    {
        // C046/C2: revoke only the session whose jti is in this token. The previous
        // code discarded a find_by_jti lookup and revoked ALL sessions, which logged
        // the user out of every device — logout means "this session" by default
        // (OWASP Session Management CS; "logout everywhere" is a separate action).
        // Revoking the jti invalidates the token because verify_bearer_token checks
        // session.is_active against the DB on every request.
        let revoked = state.session_service.revoke_session(&claims.jti).await?;
        tracing::info!(user_id = %claims.user_id(), revoked, "User session logged out");
    }
    Ok(StatusCode::NO_CONTENT)
}

/// `POST /auth/refresh` — Validates and refreshes an existing session token.
async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<serde_json::Value>> {
    let claims =
        flora_core::utils::jwt::decode_token(&req.session_token, &state.config.app.jwt_secret)
            .map_err(|_| Error::SessionExpired)?;

    // Verify session is still active in DB.
    let session_repo = PgSessionRepository::new((*state.db_pool).clone());
    let session = session_repo
        .find_by_jti(&claims.jti)
        .await?
        .ok_or(Error::SessionExpired)?;

    if !session.is_active {
        return Err(Error::SessionExpired);
    }

    // Issue a new token with same session.
    let ttl = state.config.app.session_ttl_secs;
    let new_claims = FloraClaims::new(
        session.user_id,
        claims.jti.clone(),
        session.organization_id,
        &claims.email,
        ttl,
    );
    let new_token = encode_token(&new_claims, &state.config.app.jwt_secret);

    Ok(Json(serde_json::json!({
        "session_token": new_token,
        "expires_in": ttl,
        "user_id": session.user_id.to_string()
    })))
}

/// `POST /auth/select-organization` — Bind the authenticated session to an organization.
///
/// Validates membership, revokes the current session, and issues a new
/// organization-scoped session/JWT (rotated `jti`).
async fn select_organization(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<SelectOrganizationRequest>,
) -> Result<Json<serde_json::Value>> {
    if req.organization_id.is_nil() {
        return Err(Error::Validation {
            field: "organization_id".to_string(),
            message: "organization_id must not be nil".to_string(),
        });
    }

    let session_repo = PgSessionRepository::new((*state.db_pool).clone());
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());

    let current = session_repo
        .find_by_jti(&auth.claims.jti)
        .await?
        .ok_or(Error::SessionExpired)?;
    if !current.is_active || current.user_id != auth.user_id {
        return Err(Error::SessionExpired);
    }

    let _ = membership_repo
        .find_by_user_and_organization(auth.user_id, req.organization_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member of this organization".to_string()))?;

    session_repo.revoke(current.id).await?;

    let ttl = state.config.app.session_ttl_secs;
    let jti = Uuid::now_v7().to_string();
    let now = chrono::Utc::now();
    let expires_at = now + chrono::Duration::seconds(ttl.cast_signed());
    let session = Session {
        id: Uuid::now_v7(),
        user_id: auth.user_id,
        organization_id: req.organization_id,
        jti: jti.clone(),
        refresh_token_id: None,
        client_ip: current.client_ip,
        user_agent: current.user_agent,
        expires_at,
        last_activity_at: now,
        is_active: true,
        created_at: now,
        updated_at: now,
    };
    let _session = session_repo.create(session).await?;

    let claims = FloraClaims::new(auth.user_id, jti, req.organization_id, &auth.email, ttl);
    let token = encode_token(&claims, &state.config.app.jwt_secret);

    tracing::info!(user_id = %auth.user_id, org_id = %req.organization_id, "Organization selected; session rotated");
    Ok(Json(serde_json::json!({
        "session_token": token,
        "expires_in": ttl,
        "user_id": auth.user_id.to_string(),
        "organization_id": req.organization_id.to_string()
    })))
}

/// Query parameters for the OIDC callback.
#[derive(Debug, Deserialize)]
pub struct OidcCallbackParams {
    /// The authorization code from the OIDC provider.
    pub code: Option<String>,
    /// The state parameter for CSRF/PKCE transaction binding.
    pub state: Option<String>,
    /// The original redirect URI used for authorization (validated against stored transaction).
    pub redirect_uri: Option<String>,
}
