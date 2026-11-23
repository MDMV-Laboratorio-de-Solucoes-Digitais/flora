//! Integration tests for the OIDC registration and callback flow using a local HTTP `IdP` mock.

#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
use axum::http::StatusCode;
#[cfg(test)]
use flora_api::oidc::exchange_code_and_verify_id_token;
#[cfg(test)]
use flora_api::routes::auth::create_auth_router;
#[cfg(test)]
use flora_core::config::Config;
#[cfg(test)]
use flora_core::utils::pkce::derive_s256_challenge;
#[cfg(test)]
use sqlx::PgPool;
#[cfg(test)]
use tower::ServiceExt;
#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
use crate::contract::auth::make_test_state;
#[cfg(test)]
use crate::oidc_mock::{MockAuthorizationGrant, MockOidcIdpServer};

#[cfg(test)]
async fn initiate_pkce_login(
    router: axum::Router,
) -> anyhow::Result<(String, HashMap<String, String>)> {
    let request = http::Request::builder()
        .uri("/auth/login")
        .body(axum::body::Body::empty())?;
    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
    let body: serde_json::Value = serde_json::from_slice(&bytes)?;
    let state_param = body["state"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("missing state"))?
        .to_owned();
    let auth_url = body["authorization_url"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("missing authorization_url"))?;
    let parsed = url::Url::parse(auth_url)?;
    let pairs: HashMap<String, String> = parsed.query_pairs().into_owned().collect();
    Ok((state_param, pairs))
}

#[cfg(test)]
fn mock_oidc_config(idp: &MockOidcIdpServer, client_secret: &str) -> Config {
    let mut config = Config::default();
    idp.issuer_url().clone_into(&mut config.oidc.issuer_url);
    "flora@localhost".clone_into(&mut config.oidc.client_id);
    client_secret.clone_into(&mut config.oidc.client_secret);
    config
}

#[cfg(test)]
fn require_param(pairs: &HashMap<String, String>, key: &str) -> anyhow::Result<String> {
    pairs
        .get(key)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("missing {key} in authorization_url"))
}

#[cfg(test)]
async fn run_callback_with_grant<F>(
    router: &axum::Router,
    idp: &MockOidcIdpServer,
    sub: &str,
    email: Option<&str>,
    customize: F,
) -> anyhow::Result<(StatusCode, String, String)>
where
    F: FnOnce(&mut MockAuthorizationGrant, &HashMap<String, String>) -> anyhow::Result<()>,
{
    let (oauth_state, pairs) = initiate_pkce_login(router.clone()).await?;
    let code = format!("code-{sub}-{}", Uuid::now_v7());
    let mut grant = MockAuthorizationGrant::new(
        sub,
        email.map(str::to_owned),
        None,
        require_param(&pairs, "redirect_uri")?,
        require_param(&pairs, "code_challenge")?,
        require_param(&pairs, "nonce")?,
    );
    customize(&mut grant, &pairs)?;
    idp.register_code(code.clone(), grant).await;

    let response = ServiceExt::oneshot(
        router.clone(),
        http::Request::builder()
            .uri(format!("/auth/callback?code={code}&state={oauth_state}"))
            .body(axum::body::Body::empty())?,
    )
    .await?;
    Ok((response.status(), code, oauth_state))
}

/// Exercises full HTTP OIDC PKCE exchange (`POST /oauth/v2/token` -> Discovery ->
/// asymmetric `EdDSA` JWKS -> ID token verification -> normalized identity)
/// against an ephemeral local [`MockOidcIdpServer`].
#[tokio::test]
async fn test_registration_flow_with_mock_oidc() -> anyhow::Result<()> {
    let client_secret = "mock-oidc-integration-secret";
    let idp = MockOidcIdpServer::start("flora@localhost", client_secret).await?;
    let config = mock_oidc_config(&idp, client_secret);
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let app_state = make_test_state(pool, &config);
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(app_state.clone());

    let (oauth_state, pairs) = initiate_pkce_login(router).await?;
    let redirect_uri = require_param(&pairs, "redirect_uri")?;
    let code_challenge = require_param(&pairs, "code_challenge")?;
    let nonce = require_param(&pairs, "nonce")?;
    assert_eq!(require_param(&pairs, "code_challenge_method")?, "S256");

    let code = format!("code-{}", Uuid::now_v7());
    idp.register_code(
        code.clone(),
        MockAuthorizationGrant::new(
            "oidc-sub-alice",
            Some("  Alice.Mock@Example.Test ".to_owned()),
            Some("  Alice Mock  ".to_owned()),
            redirect_uri.clone(),
            code_challenge.clone(),
            nonce,
        ),
    )
    .await;

    let transaction = app_state
        .oidc_transactions
        .consume(&oauth_state)
        .await?
        .ok_or_else(|| anyhow::anyhow!("missing stored OIDC transaction"))?;

    let identity = exchange_code_and_verify_id_token(&config.oidc, &code, &transaction).await?;
    assert_eq!(identity.sub, "oidc-sub-alice");
    assert_eq!(identity.email, "alice.mock@example.test");
    assert_eq!(identity.name, "Alice Mock");

    let requests = idp.take_token_requests().await;
    assert_eq!(requests.len(), 1);
    let req = &requests[0];
    assert_eq!(req.grant_type.as_deref(), Some("authorization_code"));
    assert_eq!(req.code.as_deref(), Some(code.as_str()));
    assert_eq!(req.client_id.as_deref(), Some("flora@localhost"));
    assert_eq!(req.client_secret.as_deref(), Some(client_secret));
    assert_eq!(req.redirect_uri.as_deref(), Some(redirect_uri.as_str()));
    let verifier = req
        .code_verifier
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("missing code_verifier sent to mock token endpoint"))?;
    assert_eq!(derive_s256_challenge(verifier), code_challenge);

    Ok(())
}

/// Exercises `/auth/callback` HTTP failure modes end-to-end against [`MockOidcIdpServer`]:
/// PKCE mismatch + single-use state consumption, tampered signature, nonce mismatch,
/// symmetric JWK rejection, missing `email` claim, and missing `id_token`.
#[tokio::test]
async fn test_registration_flow() -> anyhow::Result<()> {
    let client_secret = "mock-oidc-failure-secret";
    let idp = MockOidcIdpServer::start("flora@localhost", client_secret).await?;
    let config = mock_oidc_config(&idp, client_secret);
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(make_test_state(pool, &config));

    // 1. PKCE challenge mismatch -> 503 oidc_error, and state is single-use (replay -> 401).
    let (status_pkce, code_pkce, state_pkce) = run_callback_with_grant(
        &router,
        &idp,
        "pkce",
        Some("pkce@example.test"),
        |grant, _| {
            "wrong-pkce-challenge".clone_into(&mut grant.code_challenge);
            Ok(())
        },
    )
    .await?;
    assert_eq!(status_pkce, StatusCode::SERVICE_UNAVAILABLE);

    let replay_resp = ServiceExt::oneshot(
        router.clone(),
        http::Request::builder()
            .uri(format!(
                "/auth/callback?code={code_pkce}&state={state_pkce}"
            ))
            .body(axum::body::Body::empty())?,
    )
    .await?;
    assert_eq!(replay_resp.status(), StatusCode::UNAUTHORIZED);

    // 2. Tampered ID token signature -> 503 Service Unavailable.
    let (status_tamper, _, _) = run_callback_with_grant(
        &router,
        &idp,
        "tamper",
        Some("tamper@example.test"),
        |grant, _| {
            grant.tamper_signature = true;
            Ok(())
        },
    )
    .await?;
    assert_eq!(status_tamper, StatusCode::SERVICE_UNAVAILABLE);

    // 3. Mismatched nonce claim -> 503 Service Unavailable.
    let (status_nonce, _, _) = run_callback_with_grant(
        &router,
        &idp,
        "nonce",
        Some("nonce@example.test"),
        |grant, _| {
            "different-nonce".clone_into(&mut grant.nonce);
            Ok(())
        },
    )
    .await?;
    assert_eq!(status_nonce, StatusCode::SERVICE_UNAVAILABLE);

    // 4. Symmetric JWK published by JWKS -> 503 Service Unavailable.
    idp.set_use_symmetric_jwks(true).await;
    let (status_sym, _, _) = run_callback_with_grant(
        &router,
        &idp,
        "sym",
        Some("sym@example.test"),
        |_, _| Ok(()),
    )
    .await?;
    assert_eq!(status_sym, StatusCode::SERVICE_UNAVAILABLE);
    idp.set_use_symmetric_jwks(false).await;

    // 5. Missing email claim -> 503 Service Unavailable.
    let (status_email, _, _) =
        run_callback_with_grant(&router, &idp, "email", None, |_, _| Ok(())).await?;
    assert_eq!(status_email, StatusCode::SERVICE_UNAVAILABLE);

    // 6. Token response omitting id_token -> 503 Service Unavailable.
    let (status_omit, _, _) = run_callback_with_grant(
        &router,
        &idp,
        "omit",
        Some("omit@example.test"),
        |grant, _| {
            grant.omit_id_token = true;
            Ok(())
        },
    )
    .await?;
    assert_eq!(status_omit, StatusCode::SERVICE_UNAVAILABLE);

    Ok(())
}
