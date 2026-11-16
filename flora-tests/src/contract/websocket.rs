//! DB-gated WebSocket handshake/session contract tests (cycle 014).

#[cfg(test)]
mod db_websocket {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        routing::get,
    };
    use chrono::{Duration, Utc};
    use flora_api::{
        routes::websocket::websocket_handler,
        state::{AppState, oidc_transactions::OidcTransactionStore},
    };
    use flora_core::{
        config::Config,
        models::{Organization, Session, User},
        repositories::{
            PgFileRepository, PgMembershipRepository, PgMessageRepository,
            PgNotificationRepository, PgOrganizationRepository, PgRoleRepository,
            PgSessionRepository, PgTaskRepository, PgUserRepository, PgWorkspaceRepository,
        },
        services::{
            AuthService, ChannelService, FileService, MessageService, NotificationService,
            OrganizationService, RbacService, SessionService, TaskService, WorkspaceService,
        },
        traits::{
            ChannelRepository, FileRepository, MembershipRepository, MessageRepository,
            NotificationRepository, OrganizationRepository, RoleRepository, SessionRepository,
            TaskRepository, UserRepository, WorkspaceRepository,
        },
        utils::jwt::{self, FloraClaims},
    };
    use sqlx::PgPool;
    use std::sync::Arc;
    use tower::ServiceExt;
    use uuid::Uuid;

    fn make_test_state(pool: PgPool, config: &Config) -> AppState {
        let _user_repo: Arc<dyn UserRepository + Send + Sync> =
            Arc::new(PgUserRepository::new(pool.clone()));
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
        let channel_repo: Arc<dyn ChannelRepository + Send + Sync> = Arc::new(
            flora_core::repositories::PgChannelRepository::new(pool.clone()),
        );
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
            notification_service: Arc::new(NotificationService::new(Arc::clone(
                &notification_repo,
            ))),
            search_service: None,
            messaging_service: None,
            valkey: None,
            oidc_transactions: OidcTransactionStore::new(None),
            realtime_hub: flora_api::realtime::create_hub(),
        }
    }

    /// Builds a fully wired `AppState` over an unconnected pool: no DB, seed or server.
    fn no_db_state(config: &Config) -> anyhow::Result<AppState> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgresql://unused@localhost/unused")?;
        Ok(make_test_state(pool, config))
    }

    async fn db_state() -> anyhow::Result<Option<(AppState, Config)>> {
        let Some(url) = std::env::var("FLORA_DATABASE__POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("DATABASE_URL").ok())
        else {
            eprintln!(
                "skipping DB-backed websocket test: FLORA_DATABASE__POSTGRES_URL/DATABASE_URL not set"
            );
            return Ok(None);
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
                &format!("ws-{label}-{uniq}@example.test"),
                &format!("WS {label} User"),
            ))
            .await
            .map_err(anyhow::Error::from)
    }

    async fn seed_org(state: &AppState, label: &str) -> anyhow::Result<Organization> {
        let uniq = Uuid::now_v7().simple().to_string();
        let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
        org_repo
            .create(Organization::new(
                &format!("WS {label} {uniq}"),
                &format!("ws-{label}-{uniq}"),
            ))
            .await
            .map_err(anyhow::Error::from)
    }

    async fn seed_session(
        state: &AppState,
        user_id: Uuid,
        organization_id: Uuid,
        jti: &str,
        is_active: bool,
        expires_at: chrono::DateTime<Utc>,
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
        jwt::encode_token(
            &FloraClaims::new(
                user_id,
                jti.to_string(),
                organization_id,
                email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        )
    }

    fn ws_router(state: AppState) -> Router {
        Router::new()
            .route("/api/v1/ws", get(websocket_handler))
            .with_state(state)
    }

    async fn handshake_status(state: AppState, token: Option<&str>) -> anyhow::Result<StatusCode> {
        let router = ws_router(state);
        let mut builder = Request::builder()
            .method("GET")
            .uri("/api/v1/ws")
            .header("connection", "Upgrade")
            .header("upgrade", "websocket")
            .header("sec-websocket-version", "13")
            .header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==");
        if let Some(token) = token {
            builder = builder.header("authorization", format!("Bearer {token}"));
        }
        let request = builder.body(Body::empty())?;
        let response = ServiceExt::oneshot(router, request).await?;
        Ok(response.status())
    }

    async fn handshake_status_with_origin(
        state: AppState,
        token: Option<&str>,
        origin: Option<&str>,
    ) -> anyhow::Result<StatusCode> {
        let router = ws_router(state);
        let mut builder = Request::builder()
            .method("GET")
            .uri("/api/v1/ws")
            .header("connection", "Upgrade")
            .header("upgrade", "websocket")
            .header("sec-websocket-version", "13")
            .header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==");
        if let Some(origin) = origin {
            builder = builder.header("origin", origin);
        }
        if let Some(token) = token {
            builder = builder.header("authorization", format!("Bearer {token}"));
        }
        let request = builder.body(Body::empty())?;
        let response = ServiceExt::oneshot(router, request).await?;
        Ok(response.status())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 014)"]
    async fn db_ws_valid_session_accepted() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let user = seed_user(&state, "valid").await?;
        let org = seed_org(&state, "valid").await?;
        let jti = format!("jti-ws-valid-{}", Uuid::now_v7());
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

        let status = handshake_status(state, Some(&token)).await?;
        anyhow::ensure!(
            status == StatusCode::SWITCHING_PROTOCOLS || status == StatusCode::UPGRADE_REQUIRED,
            "expected 101 (or 426 when auth passed but upgrade extraction is not available in test harness), got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 014)"]
    async fn db_ws_revoked_session_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let user = seed_user(&state, "revoked").await?;
        let org = seed_org(&state, "revoked").await?;
        let jti = format!("jti-ws-revoked-{}", Uuid::now_v7());
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

        let status = handshake_status(state, Some(&token)).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 014)"]
    async fn db_ws_expired_session_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let user = seed_user(&state, "expired").await?;
        let org = seed_org(&state, "expired").await?;
        let jti = format!("jti-ws-expired-{}", Uuid::now_v7());
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

        let status = handshake_status(state, Some(&token)).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 014)"]
    async fn db_ws_missing_session_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let user = seed_user(&state, "missing").await?;
        let org = seed_org(&state, "missing").await?;
        let jti = format!("jti-ws-missing-{}", Uuid::now_v7());
        let token = token_for(&config, user.id, &jti, org.id, &user.email);

        let status = handshake_status(state, Some(&token)).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 014)"]
    async fn db_ws_user_mismatch_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let user = seed_user(&state, "user-a").await?;
        let other_user = seed_user(&state, "user-b").await?;
        let org = seed_org(&state, "user-mismatch").await?;
        let jti = format!("jti-ws-user-mismatch-{}", Uuid::now_v7());
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

        let status = handshake_status(state, Some(&token)).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 014)"]
    async fn db_ws_org_mismatch_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let user = seed_user(&state, "org-a").await?;
        let org = seed_org(&state, "org-a").await?;
        let other_org = seed_org(&state, "org-b").await?;
        let jti = format!("jti-ws-org-mismatch-{}", Uuid::now_v7());
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

        let status = handshake_status(state, Some(&token)).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 014)"]
    async fn db_ws_nil_org_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let user = seed_user(&state, "nil-org").await?;
        let jti = format!("jti-ws-nil-org-{}", Uuid::now_v7());
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

        let status = handshake_status(state, Some(&token)).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 014)"]
    async fn db_ws_missing_token_rejected() -> anyhow::Result<()> {
        let Some((state, _config)) = db_state().await? else {
            return Ok(());
        };

        let status = handshake_status(state, None).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    async fn ws_disallowed_origin_is_403_without_db() -> anyhow::Result<()> {
        let config = Config::default();
        let state = no_db_state(&config)?;
        let status =
            handshake_status_with_origin(state, None, Some("https://evil.example")).await?;
        anyhow::ensure!(
            status == StatusCode::FORBIDDEN,
            "expected 403, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    async fn ws_allowed_origin_passes_guard_then_fails_auth_without_db() -> anyhow::Result<()> {
        let mut config = Config::default();
        config.app.cors_origins.push("tauri://localhost".to_owned());
        let state = no_db_state(&config)?;
        let status = handshake_status_with_origin(state, None, Some("tauri://localhost")).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        Ok(())
    }
}
