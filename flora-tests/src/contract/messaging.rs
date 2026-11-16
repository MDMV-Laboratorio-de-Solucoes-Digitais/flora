//! Contract tests for messaging endpoints.

#[cfg(test)]
mod db_messaging {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::{Duration, Utc};
    use flora_api::{
        routes::channels::create_channels_router,
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

    fn make_test_state(pool: PgPool, config: &Config) -> anyhow::Result<AppState> {
        let user_repo: Arc<dyn UserRepository + Send + Sync> =
            Arc::new(PgUserRepository::new(pool.clone()));
        let _user_repo = user_repo;
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
            notification_service: Arc::new(NotificationService::new(Arc::clone(
                &notification_repo,
            ))),
            search_service: None,
            messaging_service: None,
            valkey: None,
            oidc_transactions: OidcTransactionStore::new(None),
            realtime_hub: flora_api::realtime::create_hub(),
        })
    }

    async fn db_state() -> anyhow::Result<Option<(AppState, Config)>> {
        let Some(url) = std::env::var("FLORA_DATABASE__POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("DATABASE_URL").ok())
        else {
            eprintln!(
                "skipping DB-backed messaging test: FLORA_DATABASE__POSTGRES_URL/DATABASE_URL not set"
            );
            return Ok(None);
        };

        let pool = PgPool::connect(&url).await?;
        flora_core::migrations::run_migrations(&pool).await?;
        let config = Config::default();
        let state = make_test_state(pool, &config)?;
        Ok(Some((state, config)))
    }

    struct Seed {
        user: User,
        org: Organization,
        workspace: Workspace,
        token: String,
    }

    async fn seed_member(state: &AppState, config: &Config) -> anyhow::Result<Seed> {
        let uniq = Uuid::now_v7().simple().to_string();
        let user_repo = PgUserRepository::new((*state.db_pool).clone());
        let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
        let role_repo = PgRoleRepository::new((*state.db_pool).clone());
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let workspace_repo = PgWorkspaceRepository::new((*state.db_pool).clone());
        let session_repo = PgSessionRepository::new((*state.db_pool).clone());

        let user = user_repo
            .create(User::new(
                &format!("msg-{uniq}@example.test"),
                "Messaging Test User",
            ))
            .await?;
        let org = org_repo
            .create(Organization::new(
                &format!("MsgOrg {uniq}"),
                &format!("msgorg-{uniq}"),
            ))
            .await?;
        let role = role_repo.create(Role::new(org.id, "Member")).await?;
        let _ = membership_repo
            .create(Membership::new(user.id, org.id, role.id))
            .await?;
        let workspace = workspace_repo
            .create(Workspace::new(org.id, &format!("WS {uniq}"), None))
            .await?;

        let jti = format!("jti-{}", Uuid::now_v7());
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

        Ok(Seed {
            user,
            org,
            workspace,
            token,
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

    fn channels_app(state: AppState) -> Router {
        Router::new()
            .nest("/channels", create_channels_router())
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
        body: Option<Value>,
    ) -> anyhow::Result<(StatusCode, Value)> {
        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .header("authorization", format!("Bearer {token}"));
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
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 013)"]
    async fn db_channel_and_message_rest_slice() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let app = channels_app(state.clone());

        // List empty
        let (status, body) = authed_json(
            app.clone(),
            "GET",
            format!("/channels?workspace_id={}", seed.workspace.id),
            &seed.token,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list empty: {status} {body}");
        anyhow::ensure!(
            body.as_array().is_some_and(Vec::is_empty),
            "expected empty list: {body}"
        );

        // Create channel via raw SQL to avoid sqlx enum-mapping ambiguity in this env.
        let channel_id = insert_channel(&state, seed.workspace.id, seed.org.id, "general").await?;

        // List shows channel
        let (status, listed) = authed_json(
            app.clone(),
            "GET",
            format!("/channels?workspace_id={}", seed.workspace.id),
            &seed.token,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list: {status} {listed}");
        let arr = listed
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("expected array: {listed}"))?;
        anyhow::ensure!(
            arr.iter()
                .any(|c| c["id"] == channel_id.to_string().as_str()),
            "channel not listed: {listed}"
        );

        // Post root message
        let (status, root) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{channel_id}/messages"),
            &seed.token,
            Some(json!({ "content": "hello root" })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CREATED, "post root: {status} {root}");
        let root_id = root["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing message id: {root}"))?
            .to_string();
        anyhow::ensure!(
            root["thread_id"].is_null(),
            "root thread_id should be null: {root}"
        );
        anyhow::ensure!(root["sender_id"] == seed.user.id.to_string());

        // Post threaded reply
        let (status, reply) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{channel_id}/messages"),
            &seed.token,
            Some(json!({ "content": "reply", "thread_id": root_id })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::CREATED,
            "post reply: {status} {reply}"
        );
        anyhow::ensure!(
            reply["thread_id"] == root_id,
            "thread_id not persisted: {reply}"
        );

        // List messages
        let (status, msgs) = authed_json(
            app.clone(),
            "GET",
            format!("/channels/{channel_id}/messages?limit=50"),
            &seed.token,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list msgs: {status} {msgs}");
        let arr = msgs
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("expected array: {msgs}"))?;
        anyhow::ensure!(
            arr.iter().any(|m| m["id"] == root_id),
            "root missing: {msgs}"
        );
        anyhow::ensure!(
            arr.iter().any(|m| m["thread_id"] == root_id),
            "thread reply missing: {msgs}"
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 013)"]
    async fn db_message_path_mismatch_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let ch1 = insert_channel(&state, seed.workspace.id, seed.org.id, "ch1").await?;
        let ch2 = insert_channel(&state, seed.workspace.id, seed.org.id, "ch2").await?;

        let app = channels_app(state);
        let (status, root) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{ch1}/messages"),
            &seed.token,
            Some(json!({ "content": "in ch1" })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CREATED, "post: {status} {root}");
        let msg_id = root["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing id: {root}"))?;

        let (status, _body) = authed_json(
            app,
            "PATCH",
            format!("/channels/{ch2}/messages/{msg_id}"),
            &seed.token,
            Some(json!({ "content": "hijack" })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::NOT_FOUND || status == StatusCode::FORBIDDEN,
            "expected path mismatch rejection, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 013)"]
    async fn db_cross_org_channel_access_forbidden() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed_a = seed_member(&state, &config).await?;
        let seed_b = seed_member(&state, &config).await?;
        let foreign = insert_channel(&state, seed_b.workspace.id, seed_b.org.id, "foreign").await?;

        let app = channels_app(state);
        let (status, _body) = authed_json(
            app,
            "GET",
            format!("/channels/{foreign}/messages"),
            &seed_a.token,
            None,
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::FORBIDDEN || status == StatusCode::NOT_FOUND,
            "expected cross-org rejection, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 015)"]
    async fn db_message_catch_up_after_cursor() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let channel_id = insert_channel(&state, seed.workspace.id, seed.org.id, "catchup").await?;
        let app = channels_app(state);

        let (status, m1) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{channel_id}/messages"),
            &seed.token,
            Some(json!({ "content": "first" })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CREATED, "post m1: {status} {m1}");
        let m1_id = m1["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing m1 id"))?;

        let (status, m2) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{channel_id}/messages"),
            &seed.token,
            Some(json!({ "content": "second" })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CREATED, "post m2: {status} {m2}");
        let m2_id = m2["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing m2 id"))?;

        let (status, m3) = authed_json(
            app.clone(),
            "POST",
            format!("/channels/{channel_id}/messages"),
            &seed.token,
            Some(json!({ "content": "third" })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CREATED, "post m3: {status} {m3}");
        let m3_id = m3["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing m3 id"))?;

        let (status, after) = authed_json(
            app,
            "GET",
            format!("/channels/{channel_id}/messages?after={m1_id}&limit=50"),
            &seed.token,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "catch-up: {status} {after}");
        let arr = after
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("expected array: {after}"))?;
        anyhow::ensure!(
            arr.len() >= 2,
            "expected >=2 messages after cursor: {after}"
        );
        anyhow::ensure!(
            arr[0]["id"] == m2_id,
            "expected m2 first in ASC catch-up: {after}"
        );
        anyhow::ensure!(
            arr[1]["id"] == m3_id,
            "expected m3 second in ASC catch-up: {after}"
        );
        anyhow::ensure!(
            arr.iter().all(|m| m["id"] != m1_id),
            "cursor message must be excluded: {after}"
        );

        Ok(())
    }
}
