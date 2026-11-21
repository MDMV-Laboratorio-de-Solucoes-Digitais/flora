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
    ChannelService, FileService, MessageService, NotificationService, OrganizationService,
    RbacService, SessionService, TaskService, WorkspaceService,
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
pub(crate) fn make_test_state(pool: PgPool, config: &Config) -> AppState {
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

    AppState {
        db_pool: Arc::new(pool),
        config: Arc::new(config.clone()),
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
        search_service: None,
        messaging_service: None,
        valkey: None,
        oidc_transactions: OidcTransactionStore::new(None),
        realtime_hub: flora_api::realtime::create_hub(),
    }
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
    let state = make_test_state(pool, &config);
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
    let mut config = Config::default();
    config
        .oidc
        .allowed_redirect_uris
        .push("https://app.example.com/callback".to_owned());
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config);
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
    let mut config = Config::default();
    config
        .oidc
        .allowed_redirect_uris
        .push("https://app.example.com/callback".to_owned());
    let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
    let state = make_test_state(pool, &config);
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
    let state = make_test_state(pool, &config);
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
    let state = make_test_state(pool, &config);
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
    let state = make_test_state(pool, &config);
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
    let state = make_test_state(pool, &config);
    let oauth_state = "reused-state-test";
    state
        .oidc_transactions
        .store(
            oauth_state,
            flora_api::state::oidc_transactions::OidcTransaction {
                code_verifier: "verifier".to_string(),
                redirect_uri: "http://localhost:3000/api/v1/auth/callback".to_string(),
                nonce: "test-nonce".to_string(),
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
    let state = make_test_state(pool, &config);
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

/// DB-backed select-organization tests (cycle 009).
///
/// Run explicitly with:
/// `FLORA_DATABASE__POSTGRES_URL=postgresql://... cargo test -p flora-tests contract::auth::db_ -- --ignored`
/// CI can run them in a Postgres-enabled job.
#[cfg(test)]
mod db_select_organization {
    use super::*;
    use ::chrono::{Duration, Utc};
    use flora_core::models::{Membership, Organization, Role, Session, User};
    use flora_core::repositories::{
        PgMembershipRepository, PgOrganizationRepository, PgRoleRepository, PgSessionRepository,
        PgUserRepository,
    };
    use flora_core::traits::{
        MembershipRepository, OrganizationRepository, RoleRepository, SessionRepository,
        UserRepository,
    };
    use uuid::Uuid;

    async fn db_state() -> anyhow::Result<Option<(AppState, Config)>> {
        let Some(url) = std::env::var("FLORA_DATABASE__POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("DATABASE_URL").ok())
        else {
            anyhow::bail!(
                "FLORA_DATABASE__POSTGRES_URL/DATABASE_URL required for DB contract tests"
            );
        };

        let pool = PgPool::connect(&url).await?;
        flora_core::migrations::run_migrations(&pool).await?;
        let config = Config::default();
        let state = make_test_state(pool, &config);
        Ok(Some((state, config)))
    }

    async fn seed_user_org_role(state: &AppState) -> anyhow::Result<(User, Organization, Role)> {
        let uniq = Uuid::now_v7().simple().to_string();
        let user_repo = PgUserRepository::new((*state.db_pool).clone());
        let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
        let role_repo = PgRoleRepository::new((*state.db_pool).clone());

        let user = user_repo
            .create(User::new(
                &format!("selorg-{uniq}@example.test"),
                "SelectOrg Test User",
            ))
            .await?;
        let org = org_repo
            .create(Organization::new(
                &format!("SelectOrg {uniq}"),
                &format!("selorg-{uniq}"),
            ))
            .await?;
        let role = role_repo.create(Role::new(org.id, "Member")).await?;
        Ok((user, org, role))
    }

    async fn seed_session(
        state: &AppState,
        user: &User,
        jti: &str,
        active: bool,
        session_user_id: Uuid,
    ) -> anyhow::Result<()> {
        let session_repo = PgSessionRepository::new((*state.db_pool).clone());
        let now = Utc::now();
        let _ = session_repo
            .create(Session {
                id: Uuid::now_v7(),
                user_id: session_user_id,
                organization_id: Uuid::nil(),
                jti: jti.to_string(),
                refresh_token_id: None,
                client_ip: None,
                user_agent: None,
                expires_at: now + Duration::seconds(3600),
                last_activity_at: now,
                is_active: active,
                created_at: now,
                updated_at: now,
            })
            .await?;
        let _ = user;
        Ok(())
    }

    async fn select_org_request(
        router: axum::Router,
        token: &str,
        organization_id: Uuid,
    ) -> anyhow::Result<axum::response::Response> {
        let request = http::Request::builder()
            .method("POST")
            .uri("/auth/select-organization")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {token}"))
            .body(axum::body::Body::from(format!(
                "{{\"organization_id\":\"{organization_id}\"}}"
            )))?;
        Ok(ServiceExt::oneshot(router, request).await?)
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 009)"]
    async fn db_select_organization_success_rotates_session() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, role) = seed_user_org_role(&state).await?;
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let _ = membership_repo
            .create(Membership::new(user.id, org.id, role.id))
            .await?;

        let old_jti = format!("jti-{}", Uuid::now_v7());
        seed_session(&state, &user, &old_jti, true, user.id).await?;

        let token = flora_core::utils::jwt::encode_token(
            &flora_core::utils::jwt::FloraClaims::new(
                user.id,
                old_jti.clone(),
                Uuid::nil(),
                &user.email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        );
        let router = axum::Router::new()
            .nest("/auth", create_auth_router())
            .with_state(state.clone());

        let response = select_org_request(router, &token, org.id).await?;
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
        let json: serde_json::Value = serde_json::from_slice(&body)?;
        assert_eq!(json["organization_id"], org.id.to_string());
        assert_eq!(json["user_id"], user.id.to_string());
        let new_token = json["session_token"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing session_token"))?;
        let claims = flora_core::utils::jwt::decode_token(new_token, &config.app.jwt_secret)?;
        assert_eq!(claims.org_id(), org.id);
        assert_ne!(claims.jti, old_jti);

        let session_repo = PgSessionRepository::new((*state.db_pool).clone());
        let old = session_repo.find_by_jti(&old_jti).await?;
        assert!(old.is_some_and(|s| !s.is_active));
        let new = session_repo.find_by_jti(&claims.jti).await?;
        assert!(new.is_some_and(|s| s.is_active && s.organization_id == org.id));
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 009)"]
    async fn db_select_organization_rejects_non_member() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, _role) = seed_user_org_role(&state).await?;
        let jti = format!("jti-{}", Uuid::now_v7());
        seed_session(&state, &user, &jti, true, user.id).await?;

        let token = flora_core::utils::jwt::encode_token(
            &flora_core::utils::jwt::FloraClaims::new(
                user.id,
                jti,
                Uuid::nil(),
                &user.email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        );
        let router = axum::Router::new()
            .nest("/auth", create_auth_router())
            .with_state(state);

        let response = select_org_request(router, &token, org.id).await?;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 009)"]
    async fn db_select_organization_rejects_missing_session() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, role) = seed_user_org_role(&state).await?;
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let _ = membership_repo
            .create(Membership::new(user.id, org.id, role.id))
            .await?;

        let jti = format!("jti-missing-{}", Uuid::now_v7());
        let token = flora_core::utils::jwt::encode_token(
            &flora_core::utils::jwt::FloraClaims::new(
                user.id,
                jti,
                Uuid::nil(),
                &user.email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        );
        let router = axum::Router::new()
            .nest("/auth", create_auth_router())
            .with_state(state);

        let response = select_org_request(router, &token, org.id).await?;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 009)"]
    async fn db_select_organization_rejects_inactive_session() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, role) = seed_user_org_role(&state).await?;
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let _ = membership_repo
            .create(Membership::new(user.id, org.id, role.id))
            .await?;

        let jti = format!("jti-inactive-{}", Uuid::now_v7());
        seed_session(&state, &user, &jti, false, user.id).await?;
        let token = flora_core::utils::jwt::encode_token(
            &flora_core::utils::jwt::FloraClaims::new(
                user.id,
                jti,
                Uuid::nil(),
                &user.email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        );
        let router = axum::Router::new()
            .nest("/auth", create_auth_router())
            .with_state(state);

        let response = select_org_request(router, &token, org.id).await?;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 009)"]
    async fn db_select_organization_rejects_mismatched_session_user() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, role) = seed_user_org_role(&state).await?;
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let _ = membership_repo
            .create(Membership::new(user.id, org.id, role.id))
            .await?;

        let uniq = Uuid::now_v7().simple().to_string();
        let user_repo = PgUserRepository::new((*state.db_pool).clone());
        let other_user = user_repo
            .create(User::new(
                &format!("selorg-other-{uniq}@example.test"),
                "Other Session User",
            ))
            .await?;

        let jti = format!("jti-mismatch-{}", Uuid::now_v7());
        seed_session(&state, &user, &jti, true, other_user.id).await?;
        let token = flora_core::utils::jwt::encode_token(
            &flora_core::utils::jwt::FloraClaims::new(
                user.id,
                jti,
                Uuid::nil(),
                &user.email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        );
        let router = axum::Router::new()
            .nest("/auth", create_auth_router())
            .with_state(state);

        let response = select_org_request(router, &token, org.id).await?;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        Ok(())
    }
}

/// DB-backed session-authority tests (cycle 012).
///
/// Run explicitly with:
/// `FLORA_DATABASE__POSTGRES_URL=postgresql://... cargo test -p flora-tests contract::auth::db_ -- --ignored`
/// CI runs them in the Postgres-enabled `db-tests` job.
#[cfg(test)]
mod db_session_authority {
    use super::*;
    use ::chrono::{Duration, Utc};
    use flora_api::routes::tasks::create_tasks_router;
    use flora_core::models::{Membership, Organization, Role, Session, User};
    use flora_core::traits::UserRepository;
    use uuid::Uuid;

    async fn db_state() -> anyhow::Result<Option<(AppState, Config)>> {
        let Some(url) = std::env::var("FLORA_DATABASE__POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("DATABASE_URL").ok())
        else {
            anyhow::bail!(
                "FLORA_DATABASE__POSTGRES_URL/DATABASE_URL required for DB contract tests"
            );
        };

        let pool = PgPool::connect(&url).await?;
        flora_core::migrations::run_migrations(&pool).await?;
        let config = Config::default();
        let state = make_test_state(pool, &config);
        Ok(Some((state, config)))
    }

    async fn seed_user(state: &AppState, label: &str) -> anyhow::Result<User> {
        let uniq = Uuid::now_v7().simple().to_string();
        let user_repo = PgUserRepository::new((*state.db_pool).clone());
        user_repo
            .create(User::new(
                &format!("{label}-{uniq}@example.test"),
                &format!("Session {label} User"),
            ))
            .await
            .map_err(anyhow::Error::from)
    }

    async fn seed_org(state: &AppState, label: &str) -> anyhow::Result<Organization> {
        let uniq = Uuid::now_v7().simple().to_string();
        let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
        let org = org_repo
            .create(Organization::new(
                &format!("Session {label} {uniq}"),
                &format!("session-{label}-{uniq}"),
            ))
            .await?;
        Ok(org)
    }

    async fn seed_user_org_role(state: &AppState) -> anyhow::Result<(User, Organization, Role)> {
        let user = seed_user(state, "authority").await?;
        let org = seed_org(state, "authority").await?;
        let role_repo = PgRoleRepository::new((*state.db_pool).clone());
        let role = role_repo.create(Role::new(org.id, "Member")).await?;
        Ok((user, org, role))
    }

    async fn seed_membership(
        state: &AppState,
        user: &User,
        org: &Organization,
        role: &Role,
    ) -> anyhow::Result<()> {
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let _ = membership_repo
            .create(Membership::new(user.id, org.id, role.id))
            .await?;
        Ok(())
    }

    async fn seed_session(
        state: &AppState,
        user_id: Uuid,
        organization_id: Uuid,
        jti: &str,
        is_active: bool,
        expires_at: ::chrono::DateTime<Utc>,
    ) -> anyhow::Result<()> {
        let session_repo = PgSessionRepository::new((*state.db_pool).clone());
        let now = Utc::now();
        let _ = session_repo
            .create(Session {
                id: Uuid::now_v7(),
                user_id,
                organization_id,
                jti: jti.to_string(),
                refresh_token_id: None,
                client_ip: None,
                user_agent: None,
                expires_at,
                last_activity_at: now,
                is_active,
                created_at: now,
                updated_at: now,
            })
            .await?;
        Ok(())
    }

    fn token_for(
        config: &Config,
        user_id: Uuid,
        jti: &str,
        organization_id: Uuid,
        email: &str,
    ) -> String {
        flora_core::utils::jwt::encode_token(
            &flora_core::utils::jwt::FloraClaims::new(
                user_id,
                jti.to_string(),
                organization_id,
                email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        )
    }

    async fn protected_status(state: AppState, token: &str) -> anyhow::Result<StatusCode> {
        let router = axum::Router::new()
            .nest("/api/v1/tasks", create_tasks_router())
            .with_state(state);
        let request = http::Request::builder()
            .uri("/api/v1/tasks")
            .header("authorization", format!("Bearer {token}"))
            .body(axum::body::Body::empty())?;
        let response = ServiceExt::oneshot(router, request).await?;
        Ok(response.status())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 012)"]
    async fn db_session_valid_is_accepted() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, role) = seed_user_org_role(&state).await?;
        seed_membership(&state, &user, &org, &role).await?;
        let jti = format!("jti-valid-{}", Uuid::now_v7());
        seed_session(
            &state,
            user.id,
            org.id,
            &jti,
            true,
            Utc::now() + Duration::seconds(3600),
        )
        .await?;
        let token = token_for(&config, user.id, &jti, org.id, &user.email);
        assert_eq!(protected_status(state, &token).await?, StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 012)"]
    async fn db_session_revoked_is_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, role) = seed_user_org_role(&state).await?;
        seed_membership(&state, &user, &org, &role).await?;
        let jti = format!("jti-revoked-{}", Uuid::now_v7());
        seed_session(
            &state,
            user.id,
            org.id,
            &jti,
            false,
            Utc::now() + Duration::seconds(3600),
        )
        .await?;
        let token = token_for(&config, user.id, &jti, org.id, &user.email);
        assert_eq!(
            protected_status(state, &token).await?,
            StatusCode::UNAUTHORIZED
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 012)"]
    async fn db_session_expired_is_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, role) = seed_user_org_role(&state).await?;
        seed_membership(&state, &user, &org, &role).await?;
        let jti = format!("jti-expired-{}", Uuid::now_v7());
        seed_session(
            &state,
            user.id,
            org.id,
            &jti,
            true,
            Utc::now() - Duration::seconds(1),
        )
        .await?;
        let token = token_for(&config, user.id, &jti, org.id, &user.email);
        assert_eq!(
            protected_status(state, &token).await?,
            StatusCode::UNAUTHORIZED
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 012)"]
    async fn db_session_missing_is_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, _role) = seed_user_org_role(&state).await?;
        let jti = format!("jti-missing-{}", Uuid::now_v7());
        let token = token_for(&config, user.id, &jti, org.id, &user.email);
        assert_eq!(
            protected_status(state, &token).await?,
            StatusCode::UNAUTHORIZED
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 012)"]
    async fn db_session_user_mismatch_is_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, _role) = seed_user_org_role(&state).await?;
        let other_user = seed_user(&state, "authority-other").await?;
        let jti = format!("jti-user-mismatch-{}", Uuid::now_v7());
        seed_session(
            &state,
            other_user.id,
            org.id,
            &jti,
            true,
            Utc::now() + Duration::seconds(3600),
        )
        .await?;
        let token = token_for(&config, user.id, &jti, org.id, &user.email);
        assert_eq!(
            protected_status(state, &token).await?,
            StatusCode::UNAUTHORIZED
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 012)"]
    async fn db_session_org_mismatch_is_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, _role) = seed_user_org_role(&state).await?;
        let other_org = seed_org(&state, "authority-other").await?;
        let jti = format!("jti-org-mismatch-{}", Uuid::now_v7());
        seed_session(
            &state,
            user.id,
            other_org.id,
            &jti,
            true,
            Utc::now() + Duration::seconds(3600),
        )
        .await?;
        let token = token_for(&config, user.id, &jti, org.id, &user.email);
        assert_eq!(
            protected_status(state, &token).await?,
            StatusCode::UNAUTHORIZED
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 012)"]
    async fn db_session_nil_org_passes_auth_but_requires_org() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, _org, _role) = seed_user_org_role(&state).await?;
        let jti = format!("jti-nil-org-{}", Uuid::now_v7());
        seed_session(
            &state,
            user.id,
            Uuid::nil(),
            &jti,
            true,
            Utc::now() + Duration::seconds(3600),
        )
        .await?;
        let token = token_for(&config, user.id, &jti, Uuid::nil(), &user.email);
        assert_eq!(
            protected_status(state, &token).await?,
            StatusCode::UNPROCESSABLE_ENTITY
        );
        Ok(())
    }

    /// C046/C2: logout revokes ONLY the session in the presented token.
    /// A stolen token must fail immediately (401), while another live session of
    /// the same user remains valid (logout != "logout everywhere").
    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 046)"]
    async fn db_logout_revokes_only_current_session() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (user, org, role) = seed_user_org_role(&state).await?;
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let _ = membership_repo
            .create(Membership::new(user.id, org.id, role.id))
            .await?;

        let jti_a = format!("jti-{}", Uuid::now_v7());
        let jti_b = format!("jti-{}", Uuid::now_v7());
        let exp = Utc::now() + ::chrono::Duration::seconds(3600);
        seed_session(&state, user.id, org.id, &jti_a, true, exp).await?;
        seed_session(&state, user.id, org.id, &jti_b, true, exp).await?;
        let token_a = token_for(&config, user.id, &jti_a, org.id, &user.email);
        let token_b = token_for(&config, user.id, &jti_b, org.id, &user.email);

        // Both tokens work before logout.
        anyhow::ensure!(
            protected_status(state.clone(), &token_a).await? != StatusCode::UNAUTHORIZED,
            "token A should be valid before logout"
        );
        anyhow::ensure!(
            protected_status(state.clone(), &token_b).await? != StatusCode::UNAUTHORIZED,
            "token B should be valid before logout"
        );

        // Logout with token A.
        let router = axum::Router::new()
            .nest("/auth", create_auth_router())
            .with_state(state.clone());
        let request = http::Request::builder()
            .method("POST")
            .uri("/auth/logout")
            .header("authorization", format!("Bearer {token_a}"))
            .body(axum::body::Body::empty())?;
        let response = ServiceExt::oneshot(router, request).await?;
        anyhow::ensure!(
            response.status() == StatusCode::NO_CONTENT,
            "logout should be idempotent 204: {}",
            response.status()
        );

        // A is dead (replay -> 401), B is alive (no "logout everywhere").
        anyhow::ensure!(
            protected_status(state.clone(), &token_a).await? == StatusCode::UNAUTHORIZED,
            "token A must be 401 after logout"
        );
        anyhow::ensure!(
            protected_status(state, &token_b).await? != StatusCode::UNAUTHORIZED,
            "token B must remain valid after logging out A"
        );
        Ok(())
    }
}

/// DB-backed tests for the local `POST /auth/password/login` fallback.
#[cfg(test)]
mod db_password_login {
    use super::*;
    use axum::body::{Body, to_bytes};
    use flora_core::models::User;
    use flora_core::repositories::PgUserRepository;
    use flora_core::services::PasswordService;
    use flora_core::traits::UserRepository;
    use uuid::Uuid;

    async fn db_state() -> anyhow::Result<(AppState, Config)> {
        let url = std::env::var("FLORA_DATABASE__POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("DATABASE_URL").ok())
            .ok_or_else(|| anyhow::anyhow!("DATABASE_URL required for password login tests"))?;
        let pool = PgPool::connect(&url).await?;
        flora_core::migrations::run_migrations(&pool).await?;
        let config = Config::default();
        let state = make_test_state(pool, &config);
        Ok((state, config))
    }

    async fn seed_user(
        state: &AppState,
        email: &str,
        password: Option<&str>,
    ) -> anyhow::Result<User> {
        let repo = PgUserRepository::new((*state.db_pool).clone());
        let user = repo.create(User::new(email, "Password Test")).await?;
        if let Some(password) = password {
            let hash = PasswordService::shared().hash(password)?;
            anyhow::ensure!(
                repo.set_password_hash_if_absent(user.id, &hash, chrono::Utc::now())
                    .await?,
                "password hash should have been stored"
            );
        }
        Ok(user)
    }

    async fn login(
        state: AppState,
        email: &str,
        password: &str,
    ) -> anyhow::Result<axum::response::Response> {
        let router = axum::Router::new()
            .nest("/auth", create_auth_router())
            .with_state(state);
        let payload = serde_json::json!({ "email": email, "password": password });
        let request = http::Request::builder()
            .method("POST")
            .uri("/auth/password/login")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&payload)?))?;
        Ok(ServiceExt::oneshot(router, request).await?)
    }

    async fn body(response: axum::response::Response) -> anyhow::Result<serde_json::Value> {
        let bytes = to_bytes(response.into_body(), usize::MAX).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]
    async fn db_password_login_returns_a_fresh_session() -> anyhow::Result<()> {
        let (state, config) = db_state().await?;
        let unique = Uuid::now_v7().simple();
        let email = format!("password-{unique}@example.test");
        let user = seed_user(&state, &email, Some("senha-valida-123")).await?;

        let response = login(state.clone(), &email, "senha-valida-123").await?;
        assert_eq!(response.status(), StatusCode::OK);
        let json = body(response).await?;
        assert_eq!(json["user_id"], user.id.to_string());
        assert_eq!(json["expires_in"], config.app.session_ttl_secs);
        assert_eq!(json["next_step"], "select_or_create_organization");
        let token = json["session_token"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing session_token"))?;
        let claims = flora_core::utils::jwt::decode_token(token, &config.app.jwt_secret)?;
        assert_eq!(claims.sub, user.id.to_string());
        let session = PgSessionRepository::new((*state.db_pool).clone())
            .find_by_jti(&claims.jti)
            .await?
            .ok_or_else(|| anyhow::anyhow!("session not stored"))?;
        assert!(session.is_active);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]
    async fn db_password_login_returns_the_same_401_for_wrong_password_and_unknown_user()
    -> anyhow::Result<()> {
        let (state, _config) = db_state().await?;
        let unique = Uuid::now_v7().simple();
        let email = format!("password-fail-{unique}@example.test");
        let _user = seed_user(&state, &email, Some("senha-valida-123")).await?;

        let wrong = login(state.clone(), &email, "senha-errada-123").await?;
        let unknown = login(
            state,
            &format!("desconhecido-{unique}@example.test"),
            "senha-errada-123",
        )
        .await?;
        assert_eq!(wrong.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(unknown.status(), StatusCode::UNAUTHORIZED);
        let wrong_body = body(wrong).await?;
        let unknown_body = body(unknown).await?;
        assert_eq!(wrong_body, unknown_body);
        assert_eq!(wrong_body["error"], "invalid_credentials");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]
    async fn db_password_login_rejects_users_without_a_password() -> anyhow::Result<()> {
        let (state, _config) = db_state().await?;
        let unique = Uuid::now_v7().simple();
        let email = format!("oidc-only-{unique}@example.test");
        let _user = seed_user(&state, &email, None).await?;

        let response = login(state, &email, "senha-errada-123").await?;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(body(response).await?["error"], "invalid_credentials");
        Ok(())
    }
    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]
    async fn db_password_login_rejects_inactive_account() -> anyhow::Result<()> {
        let (state, _config) = db_state().await?;
        let unique = Uuid::now_v7().simple();
        let email = format!("password-inactive-{unique}@example.test");
        let user = seed_user(&state, &email, Some("SenhaValida@12345")).await?;
        PgUserRepository::new((*state.db_pool).clone())
            .deactivate(user.id)
            .await?;

        let response = login(state, &email, "SenhaValida@12345").await?;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(body(response).await?["error"], "invalid_credentials");
        Ok(())
    }
}
