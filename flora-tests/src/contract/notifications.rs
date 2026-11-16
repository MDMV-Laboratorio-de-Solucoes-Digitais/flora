//! Contract tests for notification endpoints (019: reply-in-thread wiring).

#[cfg(test)]
mod db_notifications {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::{Duration, Utc};
    use flora_api::{
        routes::{channels::create_channels_router, notifications::create_notifications_router},
        state::{AppState, oidc_transactions::OidcTransactionStore},
    };
    use flora_core::{
        config::Config,
        models::{Membership, Organization, Role, Session, User, Workspace},
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
    use serde_json::{Value, json};
    use sqlx::PgPool;
    use std::sync::Arc;
    use tower::ServiceExt;
    use uuid::Uuid;

    fn make_test_state(pool: PgPool, config: &Config) -> AppState {
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

    struct SeedUser {
        #[expect(dead_code, reason = "seed fixture retained for symmetry/debug")]
        user: User,
        token: String,
    }

    struct Seed {
        org: Organization,
        workspace: Workspace,
        a: SeedUser,
        b: SeedUser,
    }

    async fn db_state() -> anyhow::Result<Option<(AppState, Config)>> {
        let Some(url) = std::env::var("FLORA_DATABASE__POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("DATABASE_URL").ok())
        else {
            eprintln!(
                "skipping DB-backed notification test: FLORA_DATABASE__POSTGRES_URL/DATABASE_URL not set"
            );
            return Ok(None);
        };

        let pool = PgPool::connect(&url).await?;
        flora_core::migrations::run_migrations(&pool).await?;
        let config = Config::default();
        let state = make_test_state(pool, &config);
        Ok(Some((state, config)))
    }

    async fn seed_two_members(state: &AppState, config: &Config) -> anyhow::Result<Seed> {
        let uniq = Uuid::now_v7().simple().to_string();
        let user_repo = PgUserRepository::new((*state.db_pool).clone());
        let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
        let role_repo = PgRoleRepository::new((*state.db_pool).clone());
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let workspace_repo = PgWorkspaceRepository::new((*state.db_pool).clone());
        let session_repo = PgSessionRepository::new((*state.db_pool).clone());

        let org = org_repo
            .create(Organization::new(
                &format!("NotifOrg {uniq}"),
                &format!("notiforg-{uniq}"),
            ))
            .await?;
        let role = role_repo.create(Role::new(org.id, "Member")).await?;
        let workspace = workspace_repo
            .create(Workspace::new(org.id, &format!("WS {uniq}"), None))
            .await?;

        let mut users = Vec::new();
        for label in ["a", "b"] {
            let user = user_repo
                .create(User::new(
                    &format!("notif-{label}-{uniq}@example.test"),
                    "Notif Test User",
                ))
                .await?;
            let _ = membership_repo
                .create(Membership::new(user.id, org.id, role.id))
                .await?;
            let jti = format!("jti-{}-{label}", Uuid::now_v7());
            let now = Utc::now();
            let _ = session_repo
                .create(Session {
                    id: Uuid::now_v7(),
                    user_id: user.id,
                    organization_id: org.id,
                    jti: jti.clone(),
                    refresh_token_id: None,
                    client_ip: None,
                    user_agent: None,
                    expires_at: now + Duration::seconds(3600),
                    last_activity_at: now,
                    is_active: true,
                    created_at: now,
                    updated_at: now,
                })
                .await?;
            let token = jwt::encode_token(
                &FloraClaims::new(
                    user.id,
                    jti,
                    org.id,
                    &user.email,
                    config.app.session_ttl_secs,
                ),
                &config.app.jwt_secret,
            );
            users.push(SeedUser { user, token });
        }

        let b = users.pop().ok_or_else(|| anyhow::anyhow!("seed b"))?;
        let a = users.pop().ok_or_else(|| anyhow::anyhow!("seed a"))?;
        Ok(Seed {
            org,
            workspace,
            a,
            b,
        })
    }

    async fn insert_channel(
        state: &AppState,
        workspace_id: Uuid,
        organization_id: Uuid,
        name: &str,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::now_v7();
        let _ = sqlx::query(
            "INSERT INTO channels (id, workspace_id, organization_id, name, type)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(id)
        .bind(workspace_id)
        .bind(organization_id)
        .bind(name)
        .bind("Public")
        .execute(state.db_pool.as_ref())
        .await?;
        Ok(id)
    }

    fn app(state: AppState) -> Router {
        Router::new()
            .nest("/channels", create_channels_router())
            .nest("/notifications", create_notifications_router())
            .with_state(state)
    }

    async fn json_body(resp: axum::response::Response) -> anyhow::Result<Value> {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn authed_json(
        app: Router,
        method: &str,
        uri: String,
        token: &str,
        org_id: Uuid,
        body: Option<Value>,
    ) -> anyhow::Result<(StatusCode, Value)> {
        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .header("authorization", format!("Bearer {token}"))
            .header("x-organization-id", org_id.to_string());
        let req = if let Some(body) = body {
            builder = builder.header("content-type", "application/json");
            builder.body(Body::from(serde_json::to_vec(&body)?))?
        } else {
            builder.body(Body::empty())?
        };
        let resp = ServiceExt::oneshot(app, req).await?;
        let status = resp.status();
        let json = json_body(resp).await.unwrap_or(Value::Null);
        Ok((status, json))
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 019)"]
    async fn db_reply_creates_notification_for_parent_author() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_two_members(&state, &config).await?;
        let channel_id = insert_channel(&state, seed.workspace.id, seed.org.id, "general").await?;
        let app = app(state);

        // A posts root message
        let (status, root) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{channel_id}/messages"),
            &seed.a.token,
            seed.org.id,
            Some(json!({ "content": "root by a" })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CREATED, "root post: {status} {root}");
        let root_id = root["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing root id: {root}"))?
            .to_string();

        // B replies to A
        let (status, reply) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{channel_id}/messages"),
            &seed.b.token,
            seed.org.id,
            Some(json!({ "content": "reply by b", "thread_id": root_id })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::CREATED,
            "reply post: {status} {reply}"
        );
        let reply_id = reply["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing reply id: {reply}"))?
            .to_string();

        // A receives Reply notification
        let (status, notifs) = authed_json(
            app.clone(),
            "GET",
            "/notifications?limit=20".to_string(),
            &seed.a.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list notifs: {status} {notifs}");
        let arr = notifs
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("expected array: {notifs}"))?;
        anyhow::ensure!(
            arr.iter().any(|n| n["event_type"] == "Reply"
                && n["target_id"] == reply_id
                && n["is_read"] == false),
            "reply notification for parent author missing: {notifs}"
        );

        // B (actor) should not get self-notification for replying to A
        let (status, notifs_b) = authed_json(
            app,
            "GET",
            "/notifications?limit=20".to_string(),
            &seed.b.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list b: {status} {notifs_b}");
        let arr_b = notifs_b.as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            !arr_b
                .iter()
                .any(|n| n["event_type"] == "Reply" && n["target_id"] == reply_id),
            "actor must not receive reply notification: {notifs_b}"
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 019)"]
    async fn db_self_reply_does_not_create_notification() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_two_members(&state, &config).await?;
        let channel_id = insert_channel(&state, seed.workspace.id, seed.org.id, "general").await?;
        let app = app(state);

        let (status, root) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{channel_id}/messages"),
            &seed.a.token,
            seed.org.id,
            Some(json!({ "content": "my root" })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CREATED, "root: {status} {root}");
        let root_id = root["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing root id"))?;

        let (status, reply) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{channel_id}/messages"),
            &seed.a.token,
            seed.org.id,
            Some(json!({ "content": "self reply", "thread_id": root_id })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::CREATED,
            "self reply: {status} {reply}"
        );

        let (status, notifs) = authed_json(
            app,
            "GET",
            "/notifications?limit=20".to_string(),
            &seed.a.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list: {status} {notifs}");
        let arr = notifs.as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            arr.iter().all(|n| n["event_type"] != "Reply"),
            "self-reply must not create Reply notification: {notifs}"
        );
        Ok(())
    }
}
