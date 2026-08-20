//! Contract tests for the `/auth` endpoints.

#[cfg(test)]
use axum::http::StatusCode;
#[cfg(test)]
use flora_api::routes::auth::create_auth_router;
#[cfg(test)]
use flora_api::state::AppState;
#[cfg(test)]
use flora_core::config::Config;
#[cfg(test)]
use sqlx::PgPool;
#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use tower::ServiceExt;

/// Contract test for `GET /auth/login`.
///
/// Verifies the endpoint returns a 200 OK with a JSON body containing:
/// - `authorization_url`: a valid OIDC authorize URL with correct issuer,
///   `client_id`, `response_type`, scope, and `redirect_uri` parameters.
/// - `next_step`: the string `"redirect_user_to_authorization_url"`.
#[tokio::test]
async fn test_auth_login_returns_oidc_url() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = AppState {
        db_pool: Arc::new(pool),
        config: Arc::new(config),
    };
    let router = create_auth_router().with_state(state);

    let request = http::Request::builder()
        .uri("/auth/login")
        .body(axum::body::Body::empty())?;

    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
    let body: serde_json::Value = serde_json::from_slice(&body_bytes)?;

    let auth_url = body
        .get("authorization_url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing authorization_url in response"))?;

    let next_step = body
        .get("next_step")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing next_step in response"))?;

    assert!(
        auth_url.starts_with("http://localhost:8080/oauth/v2/authorize"),
        "authorization_url must start with issuer + authorize path, got: {auth_url}"
    );
    assert!(
        auth_url.contains("client_id="),
        "authorization_url must contain client_id, got: {auth_url}"
    );
    assert!(
        auth_url.contains("response_type=code"),
        "authorization_url must contain response_type=code, got: {auth_url}"
    );
    assert!(
        auth_url.contains("scope=openid"),
        "authorization_url must contain openid scope, got: {auth_url}"
    );
    assert!(
        auth_url.contains("redirect_uri"),
        "authorization_url must contain redirect_uri, got: {auth_url}"
    );
    assert_eq!(next_step, "redirect_user_to_authorization_url");

    Ok(())
}

/// Contract test for `GET /auth/login` with custom `redirect_uri` query parameter.
///
/// Verifies that when a `redirect_uri` is provided, it is reflected in the
/// returned authorization URL.
#[tokio::test]
async fn test_auth_login_with_redirect_uri() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = AppState {
        db_pool: Arc::new(pool),
        config: Arc::new(config),
    };
    let router = create_auth_router().with_state(state);

    let request = http::Request::builder()
        .uri("/auth/login?redirect_uri=https%3A%2F%2Fapp.example.com%2Fcallback")
        .body(axum::body::Body::empty())?;

    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
    let body: serde_json::Value = serde_json::from_slice(&body_bytes)?;

    let auth_url = body
        .get("authorization_url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing authorization_url in response"))?;

    assert!(
        auth_url.contains("redirect_uri=https%3A%2F%2Fapp.example.com%2Fcallback"),
        "authorization_url must contain custom redirect_uri, got: {auth_url}"
    );

    Ok(())
}

/// Contract test for `GET /auth/login` with unencoded `redirect_uri` query parameter.
///
/// Verifies that Axum's Query extractor correctly decodes the `redirect_uri`
/// and includes it in the authorization URL.
#[tokio::test]
async fn test_auth_login_with_unencoded_redirect_uri() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = AppState {
        db_pool: Arc::new(pool),
        config: Arc::new(config),
    };
    let router = create_auth_router().with_state(state);

    let request = http::Request::builder()
        .uri("/auth/login?redirect_uri=https://app.example.com/callback")
        .body(axum::body::Body::empty())?;

    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
    let body: serde_json::Value = serde_json::from_slice(&body_bytes)?;

    let auth_url = body
        .get("authorization_url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing authorization_url in response"))?;

    // The redirect_uri should be URL-encoded in the final URL
    assert!(
        auth_url.contains("redirect_uri="),
        "authorization_url must contain redirect_uri, got: {auth_url}"
    );
    assert!(
        auth_url.contains("app.example.com"),
        "authorization_url must contain the custom domain, got: {auth_url}"
    );

    Ok(())
}
