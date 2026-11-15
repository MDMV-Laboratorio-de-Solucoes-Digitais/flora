//! Contract tests for the `/auth` endpoints.

#[cfg(test)]
use axum::http::StatusCode;
#[cfg(test)]
use flora_api::routes::auth::create_auth_router;
#[cfg(test)]
use flora_api::state::AppState;
#[cfg(test)]
use flora_api::state::oidc_transactions::OidcTransactionStore;
#[cfg(test)]
use flora_core::config::Config;
#[cfg(test)]
use flora_core::repositories::{
    PgChannelRepository, PgFileRepository, PgMembershipRepository, PgMessageRepository,
    PgNotificationRepository, PgOrganizationRepository, PgRoleRepository, PgSessionRepository,
    PgTaskRepository, PgUserRepository, PgWorkspaceRepository,
};
#[cfg(test)]
use flora_core::services::{
    AuthService, ChannelService, FileService, MessageService, NotificationService,
    OrganizationService, RbacService, SessionService, TaskService, WorkspaceService,
};
#[cfg(test)]
use flora_core::traits::{
    ChannelRepository, FileRepository, MembershipRepository, MessageRepository,
    NotificationRepository, OrganizationRepository, RoleRepository, SessionRepository,
    TaskRepository, WorkspaceRepository,
};
#[cfg(test)]
use sqlx::PgPool;
#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use tower::ServiceExt;

#[cfg(test)]
#[cfg(test)]
fn make_test_state(pool: PgPool, config: &Config) -> anyhow::Result<AppState> {
    let user_repo: Arc<dyn flora_core::traits::UserRepository + Send + Sync> =
        Arc::new(PgUserRepository::new(pool.clone()));
    let _user_repo = user_repo; // suppress unused warning
    let org_repo: Arc<dyn OrganizationRepository + Send + Sync> =
        Arc::new(PgOrganizationRepository::new(pool.clone()));
    let membership_repo: Arc<dyn MembershipRepository + Send + Sync> =
        Arc::new(PgMembershipRepository::new(pool.clone()));
    let role_repo: Arc<dyn RoleRepository + Send + Sync> =
        Arc::new(PgRoleRepository::new(pool.clone()));
    let session_repo: Arc<dyn SessionRepository + Send + Sync> =
        Arc::new(PgSessionRepository::new(pool.clone()));
    let workspace_repo: Arc<dyn WorkspaceRepository + Send + Sync> =
        Arc::new(PgWorkspaceRepository::new(pool.clone()));
    let channel_repo: Arc<dyn ChannelRepository + Send + Sync> =
        Arc::new(PgChannelRepository::new(pool.clone()));
    let message_repo: Arc<dyn MessageRepository + Send + Sync> =
        Arc::new(PgMessageRepository::new(pool.clone()));
    let task_repo: Arc<dyn TaskRepository + Send + Sync> =
        Arc::new(PgTaskRepository::new(pool.clone()));
    let file_repo: Arc<dyn FileRepository + Send + Sync> =
        Arc::new(PgFileRepository::new(pool.clone()));
    let notification_repo: Arc<dyn NotificationRepository + Send + Sync> =
        Arc::new(PgNotificationRepository::new(pool.clone()));

    Ok(AppState {
        db_pool: Arc::new(pool),
        config: Arc::new(config.clone()),
        auth_service: Arc::new(AuthService::new()),
        rbac_service: Arc::new(RbacService::new(
            Arc::clone(&role_repo),
            Arc::clone(&membership_repo),
        )),
        session_service: Arc::new(SessionService::new(Arc::clone(&session_repo))),
        organization_service: Arc::new(OrganizationService::new(
            Arc::clone(&org_repo),
            Arc::clone(&membership_repo),
            Arc::clone(&role_repo),
        )),
        workspace_service: Arc::new(WorkspaceService::new(Arc::clone(&workspace_repo))),
        channel_service: Arc::new(ChannelService::new(Arc::clone(&channel_repo))),
        message_service: Arc::new(MessageService::new(Arc::clone(&message_repo))),
        task_service: Arc::new(TaskService::new(Arc::clone(&task_repo))),
        file_service: Arc::new(FileService::new(
            Arc::clone(&file_repo),
            Arc::new(config.storage.clone()),
        )),
        notification_service: Arc::new(NotificationService::new(Arc::clone(&notification_repo))),
        search_service: Arc::new(flora_search::SearchService::new(
            &config.search.url,
            config.search.api_key.as_deref(),
            &config.search.index_template,
        )?),
        messaging_service: None,
        valkey: None,
        oidc_transactions: OidcTransactionStore::new(None),
    })
}

/// Contract test for `GET /auth/login`.
///
/// Verifies the endpoint returns a 200 OK with a JSON body containing:
/// - `authorization_url`: a valid OIDC authorize URL with correct issuer,
///   `client_id`, `response_type`, scope, `redirect_uri`, and PKCE parameters.
/// - `state`: the generated OAuth state bound to a stored transaction.
/// - `next_step`: the string `"redirect_user_to_authorization_url"`.
#[tokio::test]
async fn test_auth_login_returns_oidc_url() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config)?;
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

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
    let state_param = body
        .get("state")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing state in response"))?;
    let next_step = body
        .get("next_step")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing next_step in response"))?;

    let parsed = url::Url::parse(auth_url)?;
    let pairs: std::collections::HashMap<String, String> =
        parsed.query_pairs().into_owned().collect();

    assert_eq!(parsed.path(), "/oauth/v2/authorize");
    assert_eq!(
        pairs.get("client_id").map(String::as_str),
        Some("flora@localhost")
    );
    assert_eq!(pairs.get("response_type").map(String::as_str), Some("code"));
    assert_eq!(
        pairs.get("scope").map(String::as_str),
        Some("openid profile email")
    );
    assert_eq!(
        pairs.get("redirect_uri").map(String::as_str),
        Some("http://localhost:3000/api/v1/auth/callback")
    );
    assert_eq!(pairs.get("state").map(String::as_str), Some(state_param));
    assert!(
        pairs.contains_key("code_challenge"),
        "missing code_challenge"
    );
    assert_eq!(
        pairs.get("code_challenge_method").map(String::as_str),
        Some("S256")
    );
    assert_eq!(next_step, "redirect_user_to_authorization_url");

    Ok(())
}

/// Contract test for `GET /auth/login` with custom `redirect_uri` query parameter.
///
/// Verifies that when a `redirect_uri` is provided, it is reflected in the
/// returned authorization URL and PKCE parameters are present.
#[tokio::test]
async fn test_auth_login_with_redirect_uri() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config)?;
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

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
    let parsed = url::Url::parse(auth_url)?;
    let pairs: std::collections::HashMap<String, String> =
        parsed.query_pairs().into_owned().collect();

    assert_eq!(
        pairs.get("redirect_uri").map(String::as_str),
        Some("https://app.example.com/callback")
    );
    assert!(pairs.contains_key("state"));
    assert!(pairs.contains_key("code_challenge"));
    assert_eq!(
        pairs.get("code_challenge_method").map(String::as_str),
        Some("S256")
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
    let state = make_test_state(pool, &config)?;
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

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
    let parsed = url::Url::parse(auth_url)?;
    let pairs: std::collections::HashMap<String, String> =
        parsed.query_pairs().into_owned().collect();

    assert_eq!(
        pairs.get("redirect_uri").map(String::as_str),
        Some("https://app.example.com/callback")
    );
    assert!(pairs.contains_key("state"));
    assert!(pairs.contains_key("code_challenge"));
    assert_eq!(
        pairs.get("code_challenge_method").map(String::as_str),
        Some("S256")
    );

    Ok(())
}

/// Callback without authorization code must be rejected.
#[tokio::test]
async fn test_auth_callback_rejects_missing_code() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config)?;
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

    let request = http::Request::builder()
        .uri("/auth/callback?state=test-state")
        .body(axum::body::Body::empty())?;

    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

/// Callback without state must be rejected.
#[tokio::test]
async fn test_auth_callback_rejects_missing_state() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config)?;
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

    let request = http::Request::builder()
        .uri("/auth/callback?code=test-code")
        .body(axum::body::Body::empty())?;

    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

/// Callback with unknown/invalid state must be rejected.
#[tokio::test]
async fn test_auth_callback_rejects_invalid_state() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config)?;
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

    let request = http::Request::builder()
        .uri("/auth/callback?code=test-code&state=does-not-exist")
        .body(axum::body::Body::empty())?;

    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

/// Callback with reused state must be rejected (single-use transaction).
#[tokio::test]
async fn test_auth_callback_rejects_reused_state() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config)?;
    let oauth_state = "reused-state-test";
    state
        .oidc_transactions
        .store(
            oauth_state,
            flora_api::state::oidc_transactions::OidcTransaction {
                code_verifier: "verifier".to_string(),
                redirect_uri: "http://localhost:3000/api/v1/auth/callback".to_string(),
            },
        )
        .await?;

    // Simulate first use consuming the transaction.
    let first = state.oidc_transactions.consume(oauth_state).await?;
    assert!(
        first.is_some(),
        "stored transaction should be consumable once"
    );

    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

    let request = http::Request::builder()
        .uri(format!("/auth/callback?code=test-code&state={oauth_state}"))
        .body(axum::body::Body::empty())?;

    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

/// `POST /auth/select-organization` without Bearer token must be rejected.
#[cfg(test)]
#[tokio::test]
async fn test_select_organization_rejects_missing_auth() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config)?;
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

    let request = http::Request::builder()
        .method("POST")
        .uri("/auth/select-organization")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(
            r#"{"organization_id":"11111111-1111-1111-1111-111111111111"}"#,
        ))?;

    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

/// `POST /auth/select-organization` with nil organization must be rejected before DB access.
#[cfg(test)]
#[tokio::test]
async fn test_select_organization_rejects_nil_org() -> anyhow::Result<()> {
    let config = Config::default();
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config)?;
    let router = axum::Router::new()
        .nest("/auth", create_auth_router())
        .with_state(state);

    let claims = flora_core::utils::jwt::FloraClaims::new(
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4().to_string(),
        uuid::Uuid::nil(),
        "user@example.test",
        3600,
    );
    let token = flora_core::utils::jwt::encode_token(&claims, &config.app.jwt_secret);

    let request = http::Request::builder()
        .method("POST")
        .uri("/auth/select-organization")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {token}"))
        .body(axum::body::Body::from(
            r#"{"organization_id":"00000000-0000-0000-0000-000000000000"}"#,
        ))?;

    let response = ServiceExt::oneshot(router, request).await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}
