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
        routes::{
            channels::create_channels_router, notifications::create_notifications_router,
            tasks::create_tasks_router,
        },
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

    fn app_tasks_notifications(state: AppState) -> Router {
        Router::new()
            .nest("/tasks", create_tasks_router())
            .nest("/notifications", create_notifications_router())
            .with_state(state)
    }

    async fn seed_three_members(
        state: &AppState,
        config: &Config,
    ) -> anyhow::Result<(Seed, SeedUser)> {
        let base = seed_two_members(state, config).await?;
        let uniq = Uuid::now_v7().simple().to_string();
        let user_repo = PgUserRepository::new((*state.db_pool).clone());
        let role_repo = PgRoleRepository::new((*state.db_pool).clone());
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let session_repo = PgSessionRepository::new((*state.db_pool).clone());

        let role = role_repo
            .find_by_organization_id(base.org.id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing role"))?;
        let user = user_repo
            .create(User::new(
                &format!("notif-c-{uniq}@example.test"),
                "Notif Test User C",
            ))
            .await?;
        let _ = membership_repo
            .create(Membership::new(user.id, base.org.id, role.id))
            .await?;
        let jti = format!("jti-c-{}", Uuid::now_v7());
        let now = Utc::now();
        let _ = session_repo
            .create(Session {
                id: Uuid::now_v7(),
                user_id: user.id,
                organization_id: base.org.id,
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
                base.org.id,
                &user.email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        );
        Ok((base, SeedUser { user, token }))
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 020)"]
    #[expect(
        clippy::too_many_lines,
        reason = "020 DB-gated contract exercises create/self/no-op/reassign assignment flows end-to-end"
    )]
    async fn db_task_assignment_creates_notification_and_guards() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let (seed, c) = seed_three_members(&state, &config).await?;
        let app = app_tasks_notifications(state);
        let b_id = seed.b.user.id.to_string();

        // A creates task assigned to B -> B notified
        let (status, task) = authed_json(
            app.clone(),
            "POST",
            "/tasks".to_string(),
            &seed.a.token,
            seed.org.id,
            Some(json!({
                "title": "Assigned task",
                "workspace_id": seed.workspace.id,
                "assignee_id": b_id,
            })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::CREATED,
            "create task: {status} {task}"
        );
        let task_id = task["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing task id: {task}"))?
            .to_string();

        let (status, notifs_b) = authed_json(
            app.clone(),
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
            arr_b.iter().any(|n| n["event_type"] == "Assignment"
                && n["target_id"] == task_id
                && n["is_read"] == false),
            "assignee should receive Assignment notification: {notifs_b}"
        );

        let (status, notifs_a) = authed_json(
            app.clone(),
            "GET",
            "/notifications?limit=20".to_string(),
            &seed.a.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list a: {status} {notifs_a}");
        let arr_a = notifs_a.as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            arr_a.iter().all(|n| n["event_type"] != "Assignment"),
            "actor must not receive assignment notification: {notifs_a}"
        );

        // no-op update (same assignee provided) must not duplicate
        let (status, updated) = authed_json(
            app.clone(),
            "PATCH",
            format!("/tasks/{task_id}"),
            &seed.a.token,
            seed.org.id,
            Some(json!({ "assignee_id": b_id, "title": "Assigned task renamed" })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "noop update: {status} {updated}");
        let (status, notifs_b2) = authed_json(
            app.clone(),
            "GET",
            "/notifications?limit=20".to_string(),
            &seed.b.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list b2: {status} {notifs_b2}");
        let assign_count_b = notifs_b2
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|n| n["event_type"] == "Assignment" && n["target_id"] == task_id)
            .count();
        anyhow::ensure!(
            assign_count_b == 1,
            "no-op assignment update must not duplicate notification: {notifs_b2}"
        );

        // reassign to C -> C notified
        let c_id = c.user.id.to_string();
        let (status, reassigned) = authed_json(
            app.clone(),
            "PATCH",
            format!("/tasks/{task_id}"),
            &seed.a.token,
            seed.org.id,
            Some(json!({ "assignee_id": c_id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "reassign: {status} {reassigned}");
        let (status, notifs_c) = authed_json(
            app.clone(),
            "GET",
            "/notifications?limit=20".to_string(),
            &c.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list c: {status} {notifs_c}");
        let arr_c = notifs_c.as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            arr_c
                .iter()
                .any(|n| n["event_type"] == "Assignment" && n["target_id"] == task_id),
            "new assignee should receive Assignment notification: {notifs_c}"
        );

        // self-assignment on create must not notify actor
        let (status, self_task) = authed_json(
            app.clone(),
            "POST",
            "/tasks".to_string(),
            &seed.a.token,
            seed.org.id,
            Some(json!({
                "title": "Self assigned",
                "workspace_id": seed.workspace.id,
                "assignee_id": seed.a.user.id.to_string(),
            })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::CREATED,
            "self task: {status} {self_task}"
        );
        let self_task_id = self_task["id"].as_str().unwrap_or_default().to_string();
        let (status, notifs_a_self) = authed_json(
            app,
            "GET",
            "/notifications?limit=50".to_string(),
            &seed.a.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::OK,
            "list a self: {status} {notifs_a_self}"
        );
        let arr_a_self = notifs_a_self.as_array().cloned().unwrap_or_default();
        anyhow::ensure!(
            arr_a_self
                .iter()
                .all(|n| !(n["event_type"] == "Assignment" && n["target_id"] == self_task_id)),
            "self-assignment must not create Assignment notification: {notifs_a_self}"
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 021)"]
    #[expect(
        clippy::too_many_lines,
        reason = "021 DB-gated contract verifies unassign, dedup, and preserve flows end-to-end"
    )]
    async fn db_task_unassign_no_notification_and_unread_dedup() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_two_members(&state, &config).await?;
        let app = Router::new()
            .nest("/tasks", create_tasks_router())
            .nest("/notifications", create_notifications_router())
            .with_state(state.clone());

        // create unassigned task, then assign to B -> B notified
        let (status, created) = authed_json(
            app.clone(),
            "POST",
            "/tasks".to_string(),
            &seed.a.token,
            seed.org.id,
            Some(json!({
                "title": "Unassign flow",
                "workspace_id": seed.workspace.id,
            })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CREATED, "create: {status} {created}");
        let task_id = created["id"].as_str().unwrap_or_default().to_string();
        anyhow::ensure!(
            created["assignee_id"].is_null(),
            "initial assignee should be null: {created}"
        );

        let b_id = seed.b.user.id.to_string();
        let (status, assigned) = authed_json(
            app.clone(),
            "PATCH",
            format!("/tasks/{task_id}"),
            &seed.a.token,
            seed.org.id,
            Some(json!({ "assignee_id": b_id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "assign: {status} {assigned}");
        anyhow::ensure!(
            assigned["assignee_id"].as_str() == Some(seed.b.user.id.to_string().as_str()),
            "assignee should be B: {assigned}"
        );

        let (status, notifs_b) = authed_json(
            app.clone(),
            "GET",
            "/notifications?limit=20".to_string(),
            &seed.b.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "list b: {status} {notifs_b}");
        let arr_b = notifs_b.as_array().cloned().unwrap_or_default();
        let b_assign_count = arr_b
            .iter()
            .filter(|n| n["event_type"] == "Assignment" && n["target_id"] == task_id)
            .count();
        anyhow::ensure!(
            b_assign_count == 1,
            "initial assignment should notify B once: {notifs_b}"
        );

        // 021 dedup guard: re-send effective same assignment transition is no-op in normal flow,
        // and explicit repeat via assign->unassign->assign while unread should not duplicate.
        let (status, unassigned) = authed_json(
            app.clone(),
            "PATCH",
            format!("/tasks/{task_id}"),
            &seed.a.token,
            seed.org.id,
            Some(json!({ "assignee_id": Value::Null })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "unassign: {status} {unassigned}");
        anyhow::ensure!(
            unassigned["assignee_id"].is_null(),
            "assignee must be cleared on explicit null: {unassigned}"
        );

        let (status, notifs_b_after_unassign) = authed_json(
            app.clone(),
            "GET",
            "/notifications?limit=20".to_string(),
            &seed.b.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::OK,
            "list b after unassign: {status} {notifs_b_after_unassign}"
        );
        let b_after_unassign = notifs_b_after_unassign
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .filter(|n| n["event_type"] == "Assignment" && n["target_id"] == task_id)
            .count();
        anyhow::ensure!(
            b_after_unassign == 1,
            "unassign must not create Assignment notification: {notifs_b_after_unassign}"
        );

        // assign B again while previous unread Assignment exists -> dedup guard suppresses duplicate
        let (status, reassigned) = authed_json(
            app.clone(),
            "PATCH",
            format!("/tasks/{task_id}"),
            &seed.a.token,
            seed.org.id,
            Some(json!({ "assignee_id": b_id })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::OK,
            "reassign to B: {status} {reassigned}"
        );
        let (status, notifs_b_after_dedup) = authed_json(
            app.clone(),
            "GET",
            "/notifications?limit=20".to_string(),
            &seed.b.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::OK,
            "list b after dedup: {status} {notifs_b_after_dedup}"
        );
        let b_after_dedup = notifs_b_after_dedup
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .filter(|n| n["event_type"] == "Assignment" && n["target_id"] == task_id)
            .count();
        anyhow::ensure!(
            b_after_dedup == 1,
            "unread Assignment dedup guard must prevent duplicate: {notifs_b_after_dedup}"
        );

        // omitted assignee_id in PATCH must preserve current assignee (no-op, no duplicate)
        let (status, preserved) = authed_json(
            app.clone(),
            "PATCH",
            format!("/tasks/{task_id}"),
            &seed.a.token,
            seed.org.id,
            Some(json!({ "title": "Unassign flow renamed" })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::OK,
            "omit assignee patch: {status} {preserved}"
        );
        anyhow::ensure!(
            preserved["assignee_id"].as_str() == Some(seed.b.user.id.to_string().as_str()),
            "omitted assignee_id must preserve assignee: {preserved}"
        );
        let (status, notifs_b_final) = authed_json(
            app,
            "GET",
            "/notifications?limit=20".to_string(),
            &seed.b.token,
            seed.org.id,
            None,
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::OK,
            "list b final: {status} {notifs_b_final}"
        );
        let b_final = notifs_b_final
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .filter(|n| n["event_type"] == "Assignment" && n["target_id"] == task_id)
            .count();
        anyhow::ensure!(
            b_final == 1,
            "omit/no-op patch must not create duplicate Assignment notification: {notifs_b_final}"
        );

        Ok(())
    }
}
