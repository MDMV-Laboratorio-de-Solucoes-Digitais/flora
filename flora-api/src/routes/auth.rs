//! Authentication routes — OIDC login, logout, token refresh, and callback.
//!
//! Per T019 and T023.

use axum::{
    Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use url::form_urlencoded;
use uuid::Uuid;

use flora_core::error::Result;
use flora_core::models::Session;
use flora_core::repositories::{PgSessionRepository, PgUserRepository};
use flora_core::traits::{SessionRepository, UserRepository};
use flora_core::utils::jwt::{FloraClaims, encode_token};

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
}

/// `GET /auth/login` — Returns the OIDC authorization URL for redirect.
///
/// The frontend handles the actual browser redirect to Zitadel.
/// This endpoint returns the URL to redirect to.
async fn login(
    State(state): State<AppState>,
    Query(params): Query<AuthQuery>,
) -> Result<Json<serde_json::Value>> {
    let issuer = &state.config.oidc.issuer_url;
    let client_id = &state.config.oidc.client_id;
    let redirect_uri = params
        .redirect_uri
        .as_deref()
        .unwrap_or(&state.config.app.base_url);

    let encoded_redirect = form_urlencoded::Serializer::new(String::new())
        .append_pair("redirect_uri", redirect_uri)
        .finish();
    let auth_url = format!(
        "{}/oauth/v2/authorize?client_id={}&response_type=code&scope=openid+profile+email&{}",
        issuer.trim_end_matches('/'),
        client_id,
        encoded_redirect
    );

    tracing::info!(redirect_uri = %redirect_uri, "OIDC login initiated");

    Ok(Json(serde_json::json!({
        "authorization_url": auth_url,
        "next_step": "redirect_user_to_authorization_url"
    })))
}

/// `GET /auth/callback` — Handles the OIDC callback from Zitadel.
///
/// Exchanges the authorization code for tokens, creates or finds the user,
/// and issues a Flora session JWT.
async fn callback(
    State(state): State<AppState>,
    Query(params): Query<OidcCallbackParams>,
) -> Result<Json<serde_json::Value>> {
    let code = params
        .code
        .as_deref()
        .ok_or(flora_core::Error::Unauthorized)?;

    // Exchange code for tokens via Zitadel token endpoint
    let token_url = format!(
        "{}/oauth/v2/token",
        state.config.oidc.issuer_url.trim_end_matches('/')
    );

    let client = reqwest::Client::new();
    let redirect_uri = &state.config.app.base_url;
    let form_data = serde_urlencoded::to_string([
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", &state.config.oidc.client_id),
        ("client_secret", &state.config.oidc.client_secret),
    ])
    .map_err(|e| flora_core::Error::OidcProvider(e.to_string()))?;

    let token_response = client
        .post(&token_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()
        .await
        .map_err(|e| flora_core::Error::OidcProvider(e.to_string()))?;

    if !token_response.status().is_success() {
        return Err(flora_core::Error::OidcProvider(format!(
            "token exchange failed: {}",
            token_response.status()
        )));
    }

    let token_body: serde_json::Value = token_response
        .json()
        .await
        .map_err(|e| flora_core::Error::OidcProvider(e.to_string()))?;

    let id_token = token_body["id_token"]
        .as_str()
        .ok_or_else(|| flora_core::Error::OidcProvider("missing id_token".to_string()))?;

    // Decode the ID token (unverified — Zitadel signed it; we trust the issuer)
    // In production, verify the signature against Zitadel's JWKS.
    let claims = decode_id_token_claims(id_token)?;
    let email = claims.get("email").and_then(|v| v.as_str()).unwrap_or("");
    let name = claims
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("User");
    let sub = claims.get("sub").and_then(|v| v.as_str()).unwrap_or("");

    // Create or update user in the database
    let user_repo = PgUserRepository::new((*state.db_pool).clone());
    let user = if let Some(existing) = user_repo.find_by_email(email).await? {
        tracing::info!(user_id = %existing.id, email = %email, "Existing user logged in via OIDC");
        existing
    } else {
        let mut new_user = flora_core::models::User::new(email, name);
        new_user.oidc_subject = Some(sub.to_string());
        let created_user = user_repo.create(new_user).await?;
        tracing::info!(user_id = %created_user.id, email = %email, oidc_subject = %sub, "New user registered via OIDC");
        created_user
    };

    // Create a session record
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
        let session_repo = PgSessionRepository::new((*state.db_pool).clone());
        let _ = session_repo.find_by_jti(&claims.jti).await?;
        // Revoke all sessions for this user (security-sensitive action)
        session_repo.revoke_all_for_user(claims.user_id()).await?;
        tracing::info!(user_id = %claims.user_id(), "User session revoked");
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
            .map_err(|_| flora_core::Error::SessionExpired)?;

    // Verify session is still active in DB
    let session_repo = PgSessionRepository::new((*state.db_pool).clone());
    let session = session_repo
        .find_by_jti(&claims.jti)
        .await?
        .ok_or(flora_core::Error::SessionExpired)?;

    if !session.is_active {
        return Err(flora_core::Error::SessionExpired);
    }

    // Issue a new token with same session
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

/// Query parameters for the OIDC callback.
#[derive(Debug, Deserialize)]
pub struct OidcCallbackParams {
    /// The authorization code from the OIDC provider.
    pub code: Option<String>,
    /// The state parameter for CSRF protection.
    pub state: Option<String>,
}

/// Parses an ID token's payload (base64 JSON) without signature verification.
///
/// In production, replace with full JWKS verification via `openidconnect`.
fn decode_id_token_claims(token: &str) -> Result<serde_json::Value> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return Err(flora_core::Error::OidcProvider(
            "malformed id_token".to_string(),
        ));
    }

    let payload = parts[1];
    let padded = if payload.len().is_multiple_of(4) {
        payload.to_string()
    } else {
        format!(
            "{}{}",
            payload,
            "==".chars().take(4 - payload.len() % 4).collect::<String>()
        )
    };

    let decoded =
        base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, &padded)
            .map_err(|e| flora_core::Error::OidcProvider(e.to_string()))?;

    serde_json::from_slice(&decoded).map_err(|e| flora_core::Error::OidcProvider(e.to_string()))
}
