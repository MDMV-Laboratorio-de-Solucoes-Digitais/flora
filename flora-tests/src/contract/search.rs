//! Contract tests for search endpoints (019: PG FTS fallback).

#[cfg(test)]
mod db_search {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::{Duration, Utc};
    use flora_api::{
        routes::search::create_search_router,
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
    use serde_json::Value;
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

    struct Seed {
        user: User,
        org: Organization,
        workspace: Workspace,
        token: String,
    }

    async fn db_state() -> anyhow::Result<Option<(AppState, Config)>> {
        let Some(url) = std::env::var("FLORA_DATABASE__POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("DATABASE_URL").ok())
        else {
            eprintln!(
                "skipping DB-backed search test: FLORA_DATABASE__POSTGRES_URL/DATABASE_URL not set"
            );
            return Ok(None);
        };

        let pool = PgPool::connect(&url).await?;
        flora_core::migrations::run_migrations(&pool).await?;
        let config = Config::default();
        let state = make_test_state(pool, &config);
        Ok(Some((state, config)))
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
                &format!("search-{uniq}@example.test"),
                "Search Test User",
            ))
            .await?;
        let org = org_repo
            .create(Organization::new(
                &format!("SearchOrg {uniq}"),
                &format!("searchorg-{uniq}"),
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

    async fn insert_message(
        state: &AppState,
        channel_id: Uuid,
        organization_id: Uuid,
        sender_id: Uuid,
        content: &str,
        is_deleted: bool,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::now_v7();
        let _ = sqlx::query(
            "INSERT INTO messages (id, channel_id, organization_id, sender_id, content, is_deleted)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(id)
        .bind(channel_id)
        .bind(organization_id)
        .bind(sender_id)
        .bind(content)
        .bind(is_deleted)
        .execute(state.db_pool.as_ref())
        .await?;
        Ok(id)
    }

    fn search_app(state: AppState) -> Router {
        Router::new()
            .nest("/search", create_search_router())
            .with_state(state)
    }

    async fn json_body(resp: axum::response::Response) -> anyhow::Result<Value> {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn authed_get_json(
        app: Router,
        uri: String,
        token: &str,
        org_id: Uuid,
    ) -> anyhow::Result<(StatusCode, Value)> {
        let req = Request::builder()
            .method("GET")
            .uri(uri)
            .header("authorization", format!("Bearer {token}"))
            .header("x-organization-id", org_id.to_string())
            .body(Body::empty())?;
        let resp = ServiceExt::oneshot(app, req).await?;
        let status = resp.status();
        let json = json_body(resp).await.unwrap_or(Value::Null);
        Ok((status, json))
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 019)"]
    async fn db_search_pg_fallback_returns_matching_message() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let channel_id = insert_channel(&state, seed.workspace.id, seed.org.id, "general").await?;
        let msg_id = insert_message(
            &state,
            channel_id,
            seed.org.id,
            seed.user.id,
            "flora pg fallback unique term xyz019",
            false,
        )
        .await?;

        let app = search_app(state);
        let (status, body) = authed_get_json(
            app,
            "/search?q=xyz019".to_string(),
            &seed.token,
            seed.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "search: {status} {body}");
        anyhow::ensure!(body["query"] == "xyz019", "query echo: {body}");
        let results = body["results"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("results array missing: {body}"))?;
        anyhow::ensure!(
            results
                .iter()
                .any(|r| r["id"] == msg_id.to_string() && r["type"] == "message"),
            "matching message not found: {body}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 019)"]
    async fn db_search_empty_query_and_type_filter_behavior() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let channel_id = insert_channel(&state, seed.workspace.id, seed.org.id, "general").await?;
        let _ = insert_message(
            &state,
            channel_id,
            seed.org.id,
            seed.user.id,
            "flora filterable term abc019",
            false,
        )
        .await?;

        let app = search_app(state);

        let (status, body) = authed_get_json(
            app.clone(),
            "/search?q=%20%20%20".to_string(),
            &seed.token,
            seed.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "empty query: {status} {body}");
        anyhow::ensure!(
            body["results"].as_array().is_some_and(Vec::is_empty),
            "empty query must return no results: {body}"
        );

        // type excludes messages in 019 PG fallback (messages-only implementation)
        let (status, body) = authed_get_json(
            app,
            "/search?q=abc019&type=task".to_string(),
            &seed.token,
            seed.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "type filter: {status} {body}");
        anyhow::ensure!(
            body["results"].as_array().is_some_and(Vec::is_empty),
            "task-only filter should not return message results: {body}"
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 019)"]
    async fn db_search_tenant_isolation_and_deleted_excluded() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed_a = seed_member(&state, &config).await?;
        let seed_b = seed_member(&state, &config).await?;

        let ch_a = insert_channel(&state, seed_a.workspace.id, seed_a.org.id, "a").await?;
        let ch_b = insert_channel(&state, seed_b.workspace.id, seed_b.org.id, "b").await?;
        let visible = insert_message(
            &state,
            ch_a,
            seed_a.org.id,
            seed_a.user.id,
            "tenant visible term vis019",
            false,
        )
        .await?;
        let _deleted_msg = insert_message(
            &state,
            ch_a,
            seed_a.org.id,
            seed_a.user.id,
            "tenant deleted term del019",
            true,
        )
        .await?;
        let other_org_msg = insert_message(
            &state,
            ch_b,
            seed_b.org.id,
            seed_b.user.id,
            "tenant visible term vis019",
            false,
        )
        .await?;

        let app = search_app(state);

        let (status, body) = authed_get_json(
            app.clone(),
            "/search?q=vis019".to_string(),
            &seed_a.token,
            seed_a.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "search A: {status} {body}");
        let results = body["results"].as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            results.iter().any(|r| r["id"] == visible.to_string()),
            "own org visible message missing: {body}"
        );
        anyhow::ensure!(
            results.iter().all(|r| r["id"] != other_org_msg.to_string()),
            "cross-org leak: {body}"
        );

        let (status, body) = authed_get_json(
            app,
            "/search?q=del019".to_string(),
            &seed_a.token,
            seed_a.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "deleted search: {status} {body}");
        anyhow::ensure!(
            body["results"].as_array().is_some_and(Vec::is_empty),
            "deleted message must not be returned: {body}"
        );

        Ok(())
    }

    async fn insert_task(
        state: &AppState,
        workspace_id: Uuid,
        organization_id: Uuid,
        creator_id: Uuid,
        title: &str,
        description: Option<&str>,
        is_deleted: bool,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::now_v7();
        let _ = sqlx::query(
            "INSERT INTO tasks (id, workspace_id, organization_id, creator_id, title, description, status, is_deleted)
             VALUES ($1, $2, $3, $4, $5, $6, 'Todo', $7)",
        )
        .bind(id)
        .bind(workspace_id)
        .bind(organization_id)
        .bind(creator_id)
        .bind(title)
        .bind(description)
        .bind(is_deleted)
        .execute(state.db_pool.as_ref())
        .await?;
        Ok(id)
    }

    async fn insert_file(
        state: &AppState,
        organization_id: Uuid,
        owner_id: Uuid,
        workspace_id: Option<Uuid>,
        name: &str,
        file_type: &str,
        is_deleted: bool,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::now_v7();
        let uniq = Uuid::now_v7().simple().to_string();
        let _ = sqlx::query(
            "INSERT INTO files (id, organization_id, owner_id, workspace_id, file_type, name, size_bytes, storage_path, checksum, metadata, is_deleted)
             VALUES ($1, $2, $3, $4, $5, $6, 1, $7, $8, '{}'::jsonb, $9)",
        )
        .bind(id)
        .bind(organization_id)
        .bind(owner_id)
        .bind(workspace_id)
        .bind(file_type)
        .bind(name)
        .bind(format!("test/{uniq}"))
        .bind(format!("sum-{uniq}"))
        .bind(is_deleted)
        .execute(state.db_pool.as_ref())
        .await?;
        Ok(id)
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 020)"]
    async fn db_search_pg_fallback_returns_tasks_and_files() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let task_id = insert_task(
            &state,
            seed.workspace.id,
            seed.org.id,
            seed.user.id,
            "unique alpha task xyz020",
            Some("task description details"),
            false,
        )
        .await?;
        let file_id = insert_file(
            &state,
            seed.org.id,
            seed.user.id,
            Some(seed.workspace.id),
            "unique beta report xyz020 pdf",
            "application/pdf",
            false,
        )
        .await?;

        let app = search_app(state);

        let (status, body) = authed_get_json(
            app.clone(),
            "/search?q=alpha&type=task".to_string(),
            &seed.token,
            seed.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "task search: {status} {body}");
        let results = body["results"].as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            results
                .iter()
                .any(|r| r["id"] == task_id.to_string() && r["type"] == "task"),
            "task not found: {body}"
        );

        let (status, body) = authed_get_json(
            app.clone(),
            "/search?q=beta&type=file".to_string(),
            &seed.token,
            seed.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "file search: {status} {body}");
        let results = body["results"].as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            results
                .iter()
                .any(|r| r["id"] == file_id.to_string() && r["type"] == "file"),
            "file not found: {body}"
        );

        // omitted type can include newly supported entities
        let (status, body) = authed_get_json(
            app,
            "/search?q=xyz020".to_string(),
            &seed.token,
            seed.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "merged search: {status} {body}");
        let results = body["results"].as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            results.iter().any(|r| r["id"] == task_id.to_string()),
            "task missing in merged results: {body}"
        );
        anyhow::ensure!(
            results.iter().any(|r| r["id"] == file_id.to_string()),
            "file missing in merged results: {body}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 020)"]
    #[expect(
        clippy::too_many_lines,
        reason = "020 DB-gated contract asserts tenant/soft-delete isolation across task and file entities"
    )]
    async fn db_search_pg_fallback_tasks_files_tenant_and_deleted_isolation() -> anyhow::Result<()>
    {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed_a = seed_member(&state, &config).await?;
        let seed_b = seed_member(&state, &config).await?;

        let task_a = insert_task(
            &state,
            seed_a.workspace.id,
            seed_a.org.id,
            seed_a.user.id,
            "tenant visible task vis020",
            None,
            false,
        )
        .await?;
        let task_b = insert_task(
            &state,
            seed_b.workspace.id,
            seed_b.org.id,
            seed_b.user.id,
            "tenant visible task vis020",
            None,
            false,
        )
        .await?;
        let _task_deleted = insert_task(
            &state,
            seed_a.workspace.id,
            seed_a.org.id,
            seed_a.user.id,
            "tenant deleted task del020",
            None,
            true,
        )
        .await?;

        let file_a = insert_file(
            &state,
            seed_a.org.id,
            seed_a.user.id,
            Some(seed_a.workspace.id),
            "tenant visible file vis020 txt",
            "text/plain",
            false,
        )
        .await?;
        let file_b = insert_file(
            &state,
            seed_b.org.id,
            seed_b.user.id,
            Some(seed_b.workspace.id),
            "tenant visible file vis020 txt",
            "text/plain",
            false,
        )
        .await?;
        let _file_deleted = insert_file(
            &state,
            seed_a.org.id,
            seed_a.user.id,
            Some(seed_a.workspace.id),
            "tenant deleted file del020 txt",
            "text/plain",
            true,
        )
        .await?;

        let app = search_app(state);

        let (status, body) = authed_get_json(
            app.clone(),
            "/search?q=vis020&type=task,file".to_string(),
            &seed_a.token,
            seed_a.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "tenant search: {status} {body}");
        let results = body["results"].as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            results.iter().any(|r| r["id"] == task_a.to_string()),
            "own task missing: {body}"
        );
        anyhow::ensure!(
            results.iter().any(|r| r["id"] == file_a.to_string()),
            "own file missing: {body}"
        );
        anyhow::ensure!(
            results.iter().all(|r| r["id"] != task_b.to_string()),
            "cross-org task leak: {body}"
        );
        anyhow::ensure!(
            results.iter().all(|r| r["id"] != file_b.to_string()),
            "cross-org file leak: {body}"
        );

        let (status, body) = authed_get_json(
            app,
            "/search?q=del020&type=task,file".to_string(),
            &seed_a.token,
            seed_a.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "deleted search: {status} {body}");
        anyhow::ensure!(
            body["results"].as_array().is_some_and(Vec::is_empty),
            "deleted task/file must not be returned: {body}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 021)"]
    async fn db_search_pg_fallback_type_normalization_and_limit_bounds() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let channel_id = insert_channel(&state, seed.workspace.id, seed.org.id, "general").await?;
        let msg_id = insert_message(
            &state,
            channel_id,
            seed.org.id,
            seed.user.id,
            "hardening token edge021 appears in message",
            false,
        )
        .await?;
        let task_id = insert_task(
            &state,
            seed.workspace.id,
            seed.org.id,
            seed.user.id,
            "hardening token edge021 task",
            None,
            false,
        )
        .await?;
        let file_id = insert_file(
            &state,
            seed.org.id,
            seed.user.id,
            Some(seed.workspace.id),
            "hardening edge021 txt",
            "text/plain",
            false,
        )
        .await?;

        let app = search_app(state);

        // mixed case / plural / CSV tokens
        let (status, body) = authed_get_json(
            app.clone(),
            "/search?q=edge021&type=Messages,%20TASKS%20,files".to_string(),
            &seed.token,
            seed.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "mixed types: {status} {body}");
        let results = body["results"].as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            results
                .iter()
                .any(|r| r["id"] == msg_id.to_string() && r["type"] == "message"),
            "message missing for mixed types: {body}"
        );
        anyhow::ensure!(
            results
                .iter()
                .any(|r| r["id"] == task_id.to_string() && r["type"] == "task"),
            "task missing for mixed types: {body}"
        );
        anyhow::ensure!(
            results
                .iter()
                .any(|r| r["id"] == file_id.to_string() && r["type"] == "file"),
            "file missing for mixed types: {body}"
        );

        // unknown-only type tokens -> empty
        let (status, body) = authed_get_json(
            app.clone(),
            "/search?q=edge021&type=unknown,other".to_string(),
            &seed.token,
            seed.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "unknown types: {status} {body}");
        anyhow::ensure!(
            body["results"].as_array().is_some_and(Vec::is_empty),
            "unknown-only types must return empty: {body}"
        );

        // explicit high limit is capped server-side (contract: <=50); response stays valid/bounded
        let (status, body) = authed_get_json(
            app,
            "/search?q=edge021&limit=999".to_string(),
            &seed.token,
            seed.org.id,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "limit cap: {status} {body}");
        let results = body["results"].as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            results.len() <= 50,
            "results must respect server cap: {body}"
        );
        Ok(())
    }
}
