//! Contract tests for file management endpoints (018: DB-gated, slice B hardening).

#[cfg(test)]
mod db_files {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode, header},
    };
    use chrono::{Duration, Utc};
    use flora_api::{
        routes::files::create_files_router,
        state::{AppState, oidc_transactions::OidcTransactionStore},
    };
    use flora_core::{
        config::Config,
        models::{Membership, Organization, Pagination, Role, Session, User, Workspace},
        repositories::{
            PgFileRepository, PgMembershipRepository, PgMessageRepository,
            PgNotificationRepository, PgOrganizationRepository, PgRoleRepository,
            PgSessionRepository, PgTaskRepository, PgUserRepository, PgWorkspaceRepository,
        },
        services::{
            ChannelService, FileService, MessageService, NotificationService, OrganizationService,
            RbacService, SessionService, TaskService, WorkspaceService,
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

    fn multipart_body(
        boundary: &str,
        fields: &[(&str, &str)],
        file: Option<(&str, &str, &str, &[u8])>,
    ) -> Vec<u8> {
        let mut body = Vec::new();
        for (name, value) in fields {
            body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
            body.extend_from_slice(
                format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
            );
            body.extend_from_slice(value.as_bytes());
            body.extend_from_slice(b"\r\n");
        }
        if let Some((name, filename, content_type, data)) = file {
            body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
            body.extend_from_slice(
                format!(
                    "Content-Disposition: form-data; name=\"{name}\"; filename=\"{filename}\"\r\n"
                )
                .as_bytes(),
            );
            body.extend_from_slice(format!("Content-Type: {content_type}\r\n\r\n").as_bytes());
            body.extend_from_slice(data);
            body.extend_from_slice(b"\r\n");
        }
        body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
        body
    }

    fn make_test_state(pool: PgPool, config: &Config) -> AppState {
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
        #[expect(dead_code, reason = "seed fixture retained for symmetry/debug")]
        user: User,
        org: Organization,
        workspace: Workspace,
        token: String,
        #[expect(dead_code, reason = "seed fixture retained for symmetry/debug")]
        user_id: Uuid,
    }

    async fn db_state_with_config(
        configure: impl FnOnce(&mut Config),
    ) -> anyhow::Result<Option<(AppState, Config)>> {
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
        let mut config = Config::default();
        let uniq = Uuid::now_v7().simple().to_string();
        config.storage.local_path = format!("/tmp/flora-test-storage-{uniq}");
        config.storage.max_upload_bytes = 1024 * 1024;
        configure(&mut config);
        let state = make_test_state(pool, &config);
        Ok(Some((state, config)))
    }

    async fn db_state() -> anyhow::Result<Option<(AppState, Config)>> {
        db_state_with_config(|_| {}).await
    }

    async fn seed_user_org_workspace(state: &AppState, config: &Config) -> anyhow::Result<Seed> {
        seed_user_org_workspace_in_org(state, config, None).await
    }

    async fn seed_user_org_workspace_in_org(
        state: &AppState,
        config: &Config,
        existing_org: Option<Organization>,
    ) -> anyhow::Result<Seed> {
        let uniq = Uuid::now_v7().simple().to_string();
        let user_repo = PgUserRepository::new((*state.db_pool).clone());
        let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
        let role_repo = PgRoleRepository::new((*state.db_pool).clone());
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let session_repo = PgSessionRepository::new((*state.db_pool).clone());
        let workspace_repo = PgWorkspaceRepository::new((*state.db_pool).clone());

        let user = user_repo
            .create(User::new(
                &format!("file-{uniq}@example.test"),
                "File Test User",
            ))
            .await?;

        let org = if let Some(org) = existing_org {
            org
        } else {
            org_repo
                .create(Organization::new(
                    &format!("FileOrg {uniq}"),
                    &format!("fileorg-{uniq}"),
                ))
                .await?
        };

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
            user_id: user.id,
            user,
            org,
            workspace,
            token,
        })
    }

    fn files_app(state: AppState) -> Router {
        Router::new()
            .nest("/api/v1/files", create_files_router())
            .with_state(state)
    }

    async fn json_body(resp: axum::response::Response) -> anyhow::Result<Value> {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn count_files_in_dir(path: &std::path::Path) -> anyhow::Result<usize> {
        if !tokio::fs::try_exists(path).await? {
            return Ok(0);
        }
        let mut entries = tokio::fs::read_dir(path).await?;
        let mut count = 0;
        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_file() {
                count += 1;
            }
        }
        Ok(count)
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 018)"]
    async fn db_upload_download_and_get_metadata_happy_path() -> anyhow::Result<()> {
        let Some((state, _config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_user_org_workspace(&state, state.config.as_ref()).await?;
        let app = files_app(state);

        let boundary = "flora-test-boundary";
        let body = multipart_body(
            boundary,
            &[],
            Some(("file", "hello.txt", "text/plain", b"hello flora")),
        );
        let req = Request::builder()
            .method("POST")
            .uri(format!(
                "/api/v1/files/upload?workspace_id={}",
                seed.workspace.id
            ))
            .header("authorization", format!("Bearer {}", seed.token))
            .header("x-organization-id", seed.org.id.to_string())
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))?;

        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created = json_body(resp).await?;
        assert_eq!(created["fileName"], "hello.txt");
        assert_eq!(created["mimeType"], "text/plain");
        assert_eq!(created["byteSize"], 11);
        let file_id = created["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing id"))?
            .to_string();

        let req = Request::builder()
            .method("GET")
            .uri(format!("/api/v1/files/{file_id}"))
            .header("authorization", format!("Bearer {}", seed.token))
            .header("x-organization-id", seed.org.id.to_string())
            .body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        let meta = json_body(resp).await?;
        assert_eq!(meta["id"], file_id);
        assert_eq!(meta["name"], "hello.txt");
        assert_eq!(meta["workspace_id"], seed.workspace.id.to_string());

        let req = Request::builder()
            .method("GET")
            .uri(format!("/api/v1/files/{file_id}/download"))
            .header("authorization", format!("Bearer {}", seed.token))
            .header("x-organization-id", seed.org.id.to_string())
            .body(Body::empty())?;
        let resp = app.oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok()),
            Some("text/plain")
        );
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await?;
        assert_eq!(bytes.as_ref(), b"hello flora");

        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 018)"]
    async fn db_upload_rejects_workspace_not_in_org() -> anyhow::Result<()> {
        let Some((state, _config)) = db_state().await? else {
            return Ok(());
        };
        let seed_a = seed_user_org_workspace(&state, state.config.as_ref()).await?;
        // create another org/workspace
        let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
        let workspace_repo = PgWorkspaceRepository::new((*state.db_pool).clone());
        let uniq = Uuid::now_v7().simple().to_string();
        let org_b = org_repo
            .create(Organization::new(
                &format!("OtherOrg {uniq}"),
                &format!("otherorg-{uniq}"),
            ))
            .await?;
        let ws_b = workspace_repo
            .create(Workspace::new(org_b.id, &format!("WSB {uniq}"), None))
            .await?;

        let app = files_app(state);
        let boundary = "flora-test-boundary";
        let body = multipart_body(
            boundary,
            &[],
            Some(("file", "hello.txt", "text/plain", b"hello")),
        );
        let req = Request::builder()
            .method("POST")
            .uri(format!("/api/v1/files/upload?workspace_id={}", ws_b.id))
            .header("authorization", format!("Bearer {}", seed_a.token))
            .header("x-organization-id", seed_a.org.id.to_string())
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))?;
        let resp = app.oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 018)"]
    async fn db_cross_org_cannot_read_download_delete() -> anyhow::Result<()> {
        let Some((state, _config)) = db_state().await? else {
            return Ok(());
        };
        let seed_a = seed_user_org_workspace(&state, state.config.as_ref()).await?;
        let seed_b = seed_user_org_workspace(&state, state.config.as_ref()).await?;
        let app = files_app(state);

        let boundary = "flora-test-boundary";
        let body = multipart_body(
            boundary,
            &[],
            Some(("file", "a.txt", "text/plain", b"secret")),
        );
        let req = Request::builder()
            .method("POST")
            .uri(format!(
                "/api/v1/files/upload?workspace_id={}",
                seed_a.workspace.id
            ))
            .header("authorization", format!("Bearer {}", seed_a.token))
            .header("x-organization-id", seed_a.org.id.to_string())
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created = json_body(resp).await?;
        let file_id = created["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing id"))?;

        let req = Request::builder()
            .method("GET")
            .uri(format!("/api/v1/files/{file_id}"))
            .header("authorization", format!("Bearer {}", seed_b.token))
            .header("x-organization-id", seed_b.org.id.to_string())
            .body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let req = Request::builder()
            .method("GET")
            .uri(format!("/api/v1/files/{file_id}/download"))
            .header("authorization", format!("Bearer {}", seed_b.token))
            .header("x-organization-id", seed_b.org.id.to_string())
            .body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/api/v1/files/{file_id}"))
            .header("authorization", format!("Bearer {}", seed_b.token))
            .header("x-organization-id", seed_b.org.id.to_string())
            .body(Body::empty())?;
        let resp = app.oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 018)"]
    async fn db_upload_requires_workspace_and_rejects_empty_file() -> anyhow::Result<()> {
        let Some((state, _config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_user_org_workspace(&state, state.config.as_ref()).await?;
        let app = files_app(state);

        // missing workspace_id
        let boundary = "flora-test-boundary";
        let body = multipart_body(boundary, &[], Some(("file", "a.txt", "text/plain", b"a")));
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/files/upload")
            .header("authorization", format!("Bearer {}", seed.token))
            .header("x-organization-id", seed.org.id.to_string())
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);

        // empty file
        let body = multipart_body(
            boundary,
            &[],
            Some(("file", "empty.txt", "text/plain", b"")),
        );
        let req = Request::builder()
            .method("POST")
            .uri(format!(
                "/api/v1/files/upload?workspace_id={}",
                seed.workspace.id
            ))
            .header("authorization", format!("Bearer {}", seed.token))
            .header("x-organization-id", seed.org.id.to_string())
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))?;
        let resp = app.oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 018)"]
    async fn db_delete_owner_and_non_owner_behavior() -> anyhow::Result<()> {
        let Some((state, _config)) = db_state().await? else {
            return Ok(());
        };
        let seed_owner = seed_user_org_workspace(&state, state.config.as_ref()).await?;
        let seed_other = seed_user_org_workspace_in_org(
            &state,
            state.config.as_ref(),
            Some(seed_owner.org.clone()),
        )
        .await?;
        let app = files_app(state);

        let boundary = "flora-test-boundary";
        let body = multipart_body(
            boundary,
            &[],
            Some(("file", "del.txt", "text/plain", b"bye")),
        );
        let req = Request::builder()
            .method("POST")
            .uri(format!(
                "/api/v1/files/upload?workspace_id={}",
                seed_owner.workspace.id
            ))
            .header("authorization", format!("Bearer {}", seed_owner.token))
            .header("x-organization-id", seed_owner.org.id.to_string())
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created = json_body(resp).await?;
        let file_id = created["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing id"))?;

        // non-owner in same org cannot delete
        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/api/v1/files/{file_id}"))
            .header("authorization", format!("Bearer {}", seed_other.token))
            .header("x-organization-id", seed_owner.org.id.to_string())
            .body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);

        // owner can delete
        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/api/v1/files/{file_id}"))
            .header("authorization", format!("Bearer {}", seed_owner.token))
            .header("x-organization-id", seed_owner.org.id.to_string())
            .body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        // deleted file not found/readable
        let req = Request::builder()
            .method("GET")
            .uri(format!("/api/v1/files/{file_id}"))
            .header("authorization", format!("Bearer {}", seed_owner.token))
            .header("x-organization-id", seed_owner.org.id.to_string())
            .body(Body::empty())?;
        let resp = app.oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 018)"]
    async fn db_legacy_metadata_upload_validates_workspace_in_org() -> anyhow::Result<()> {
        let Some((state, _config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_user_org_workspace(&state, state.config.as_ref()).await?;
        let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
        let workspace_repo = PgWorkspaceRepository::new((*state.db_pool).clone());
        let uniq = Uuid::now_v7().simple().to_string();
        let org_b = org_repo
            .create(Organization::new(
                &format!("OrgB {uniq}"),
                &format!("orgb-{uniq}"),
            ))
            .await?;
        let ws_b = workspace_repo
            .create(Workspace::new(org_b.id, &format!("WSB {uniq}"), None))
            .await?;

        let app = files_app(state);
        let req = Request::builder()
            .method("POST")
            .uri("/api/v1/files")
            .header("authorization", format!("Bearer {}", seed.token))
            .header("x-organization-id", seed.org.id.to_string())
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::json!({
                    "workspace_id": ws_b.id,
                    "name": "meta.txt",
                    "file_type": "text/plain",
                    "size_bytes": 10
                })
                .to_string(),
            ))?;
        let resp = app.oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
        Ok(())
    }

    async fn authed_json_request(
        app: &Router,
        method: &str,
        uri: &str,
        token: &str,
        org_id: Uuid,
        expected_status: StatusCode,
    ) -> anyhow::Result<Value> {
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header("authorization", format!("Bearer {token}"))
            .header("x-organization-id", org_id.to_string())
            .body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), expected_status);
        if expected_status == StatusCode::NO_CONTENT {
            return Ok(Value::Null);
        }
        json_body(resp).await
    }

    async fn upload_test_file(
        app: &Router,
        token: &str,
        org_id: Uuid,
        workspace_id: Uuid,
        filename: &str,
        contents: &[u8],
    ) -> anyhow::Result<String> {
        let boundary = "flora-list-boundary";
        let body = multipart_body(
            boundary,
            &[],
            Some(("file", filename, "text/plain", contents)),
        );
        let req = Request::builder()
            .method("POST")
            .uri(format!("/api/v1/files/upload?workspace_id={workspace_id}"))
            .header("authorization", format!("Bearer {token}"))
            .header("x-organization-id", org_id.to_string())
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(body))?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created = json_body(resp).await?;
        created["id"]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| anyhow::anyhow!("missing uploaded file id for {filename}"))
    }

    async fn setup_extra_list_workspaces(
        pool: &PgPool,
        org_id: Uuid,
    ) -> anyhow::Result<(Uuid, Uuid)> {
        let org_repo = PgOrganizationRepository::new(pool.clone());
        let workspace_repo = PgWorkspaceRepository::new(pool.clone());
        let uniq = Uuid::now_v7().simple().to_string();

        let ws_same_org_2 = workspace_repo
            .create(Workspace::new(org_id, &format!("WS2 {uniq}"), None))
            .await?;
        let org_b = org_repo
            .create(Organization::new(
                &format!("OrgB {uniq}"),
                &format!("orgb-{uniq}"),
            ))
            .await?;
        let ws_other_org = workspace_repo
            .create(Workspace::new(org_b.id, &format!("WSB {uniq}"), None))
            .await?;
        Ok((ws_same_org_2.id, ws_other_org.id))
    }

    async fn list_workspace_files(
        app: &Router,
        token: &str,
        org_id: Uuid,
        workspace_id: Uuid,
    ) -> anyhow::Result<Vec<Value>> {
        let uri = format!("/api/v1/files?workspace_id={workspace_id}");
        let val = authed_json_request(app, "GET", &uri, token, org_id, StatusCode::OK).await?;
        val.as_array()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("expected array for workspace {workspace_id}"))
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 115)"]
    async fn db_list_files_scoped_to_workspace_and_org() -> anyhow::Result<()> {
        let Some((state, _config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_user_org_workspace(&state, state.config.as_ref()).await?;
        let (ws_same_org_2_id, ws_other_org_id) =
            setup_extra_list_workspaces(&state.db_pool, seed.org.id).await?;
        let app = files_app(state);

        // Missing workspace_id -> 422 validation_error
        let missing_err = authed_json_request(
            &app,
            "GET",
            "/api/v1/files",
            &seed.token,
            seed.org.id,
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await?;
        assert_eq!(missing_err["error"], "validation_error");
        assert_eq!(missing_err["field"], "workspace_id");

        // Cross-org and non-existent workspaces both return identical 422 validation_error (anti-enumeration BOLA guard)
        let cross_uri = format!("/api/v1/files?workspace_id={ws_other_org_id}");
        let cross_err = authed_json_request(
            &app,
            "GET",
            &cross_uri,
            &seed.token,
            seed.org.id,
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await?;
        let ghost_uri = format!("/api/v1/files?workspace_id={}", Uuid::now_v7());
        let ghost_err = authed_json_request(
            &app,
            "GET",
            &ghost_uri,
            &seed.token,
            seed.org.id,
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await?;
        assert_eq!(cross_err["error"], "validation_error");
        assert_eq!(cross_err["field"], "workspace_id");
        assert_eq!(cross_err, ghost_err);

        // Upload alpha.txt to workspace 1 and beta.txt to workspace 2
        let id_a = upload_test_file(
            &app,
            &seed.token,
            seed.org.id,
            seed.workspace.id,
            "alpha.txt",
            b"alpha",
        )
        .await?;
        let _id_b = upload_test_file(
            &app,
            &seed.token,
            seed.org.id,
            ws_same_org_2_id,
            "beta.txt",
            b"beta",
        )
        .await?;

        // Workspace 1 lists only alpha.txt with exact 6 DTO keys (no storage_path/metadata leak)
        let items_a =
            list_workspace_files(&app, &seed.token, seed.org.id, seed.workspace.id).await?;
        assert_eq!(items_a.len(), 1);
        assert_eq!(items_a[0].as_object().map(serde_json::Map::len), Some(6));
        assert_eq!(items_a[0]["id"], id_a);
        assert_eq!(items_a[0]["fileName"], "alpha.txt");
        assert_eq!(items_a[0]["mimeType"], "text/plain");
        assert_eq!(items_a[0]["byteSize"], 5);
        assert_eq!(
            items_a[0]["rustFsUrl"],
            format!("/api/v1/files/download/{id_a}")
        );
        assert!(items_a[0].get("storage_path").is_none());

        // Workspace 2 lists only beta.txt
        let items_b =
            list_workspace_files(&app, &seed.token, seed.org.id, ws_same_org_2_id).await?;
        assert_eq!(items_b.len(), 1);
        assert_eq!(items_b[0]["fileName"], "beta.txt");

        // Soft-delete alpha.txt -> omitted from workspace 1 listing
        let del_uri = format!("/api/v1/files/{id_a}");
        let _ = authed_json_request(
            &app,
            "DELETE",
            &del_uri,
            &seed.token,
            seed.org.id,
            StatusCode::NO_CONTENT,
        )
        .await?;
        let items_after =
            list_workspace_files(&app, &seed.token, seed.org.id, seed.workspace.id).await?;
        assert_eq!(items_after, Vec::<Value>::new());

        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 126)"]
    async fn db_aggregate_quota_rejects_second_upload() -> anyhow::Result<()> {
        // 10-byte aggregate quota and two 6-byte uploads: the first is accepted,
        // the second would bring the active total to 12 bytes and must be rejected.
        let Some((state, config)) = db_state_with_config(|config| {
            config.storage.org_quota_bytes = 10;
            config.storage.workspace_quota_bytes = 10;
        })
        .await?
        else {
            return Ok(());
        };
        let seed = seed_user_org_workspace(&state, state.config.as_ref()).await?;
        let pool = (*state.db_pool).clone();
        let app = files_app(state);

        let boundary = "flora-quota-boundary";
        let first_body = multipart_body(
            boundary,
            &[],
            Some(("file", "first.txt", "text/plain", b"abcdef")),
        );
        let req = Request::builder()
            .method("POST")
            .uri(format!(
                "/api/v1/files/upload?workspace_id={}",
                seed.workspace.id
            ))
            .header("authorization", format!("Bearer {}", seed.token))
            .header("x-organization-id", seed.org.id.to_string())
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(first_body))?;
        let resp = app.clone().oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let created = json_body(resp).await?;
        let first_id = created["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing id for accepted upload"))?
            .to_string();

        let second_body = multipart_body(
            boundary,
            &[],
            Some(("file", "second.txt", "text/plain", b"ghijkl")),
        );
        let req = Request::builder()
            .method("POST")
            .uri(format!(
                "/api/v1/files/upload?workspace_id={}",
                seed.workspace.id
            ))
            .header("authorization", format!("Bearer {}", seed.token))
            .header("x-organization-id", seed.org.id.to_string())
            .header(
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(Body::from(second_body))?;
        let resp = app.oneshot(req).await?;
        assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
        let rejected = json_body(resp).await?;
        assert_eq!(rejected["error"], "file_too_large");

        let file_repo = PgFileRepository::new(pool);
        let page = file_repo
            .find_by_workspace_id(
                seed.org.id,
                seed.workspace.id,
                Pagination {
                    limit: 100,
                    offset: 0,
                },
            )
            .await?;
        assert_eq!(page.items.len(), 1, "only the accepted upload is persisted");
        assert_eq!(page.items[0].id.to_string(), first_id);

        let root = std::path::Path::new(config.storage.local_path.as_str());
        assert_eq!(
            count_files_in_dir(&root.join(".tmp")).await?,
            0,
            "no orphan .part blob in configured temp storage"
        );
        let files_dir = root
            .join("orgs")
            .join(seed.org.id.to_string())
            .join("workspaces")
            .join(seed.workspace.id.to_string())
            .join("files");
        assert_eq!(
            count_files_in_dir(&files_dir).await?,
            1,
            "only the accepted upload has a stored blob"
        );
        Ok(())
    }
}
