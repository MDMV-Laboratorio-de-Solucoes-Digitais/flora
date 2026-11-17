//! DB-gated contract tests for task endpoints (cycle 044).
//! Replaces the previous `Ok(())` stubs with real assertions on the Axum router
//! backed by disposable PostgreSQL 18 (the `db-tests` CI leg).

#[cfg(test)]
mod db {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::{Duration, Utc};
    use flora_api::routes::tasks::create_tasks_router;
    use flora_api::state::AppState;
    use flora_core::config::Config;
    use flora_core::migrations::run_migrations;
    use flora_core::models::{Membership, Organization, Role, Session, User, Workspace};
    use flora_core::repositories::{
        PgMembershipRepository, PgOrganizationRepository, PgRoleRepository, PgSessionRepository,
        PgUserRepository, PgWorkspaceRepository,
    };
    use flora_core::traits::{
        MembershipRepository, OrganizationRepository, RoleRepository, SessionRepository,
        UserRepository, WorkspaceRepository,
    };
    use flora_core::utils::jwt::{FloraClaims, encode_token};
    use serde_json::{Value, json};
    use sqlx::PgPool;
    use tower::ServiceExt;

    use super::super::auth::make_test_state;

    struct Seed {
        user_id: String,
        workspace_id: String,
        token: String,
    }

    async fn db_state() -> anyhow::Result<Option<(AppState, Config)>> {
        let Some(url) = std::env::var("FLORA_DATABASE__POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("DATABASE_URL").ok())
        else {
            return Ok(None);
        };
        let pool = PgPool::connect(&url).await?;
        run_migrations(&pool).await?;
        let config = Config::default();
        Ok(Some((make_test_state(pool, &config), config)))
    }

    async fn seed_member(state: &AppState, config: &Config) -> anyhow::Result<Seed> {
        let uniq = uuid::Uuid::now_v7().simple().to_string();
        let user = PgUserRepository::new((*state.db_pool).clone())
            .create(User::new(&format!("task-{uniq}@example.test"), "Task User"))
            .await?;
        let org = PgOrganizationRepository::new((*state.db_pool).clone())
            .create(Organization::new(
                &format!("TaskOrg {uniq}"),
                &format!("taskorg-{uniq}"),
            ))
            .await?;
        let role = PgRoleRepository::new((*state.db_pool).clone())
            .create(Role::new(org.id, "Owner"))
            .await?;
        let _m = PgMembershipRepository::new((*state.db_pool).clone())
            .create(Membership::new(user.id, org.id, role.id))
            .await?;
        let ws = PgWorkspaceRepository::new((*state.db_pool).clone())
            .create(Workspace::new(org.id, &format!("WS {uniq}"), None))
            .await?;
        let jti = format!("jti-{}", uuid::Uuid::now_v7());
        let now = Utc::now();
        let _s = PgSessionRepository::new((*state.db_pool).clone())
            .create(Session {
                id: uuid::Uuid::now_v7(),
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
        let token = encode_token(
            &FloraClaims::new(
                user.id,
                jti,
                org.id,
                &user.email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        );
        let _ = &org; // org is used indirectly via membership/session/workspace
        Ok(Seed {
            user_id: user.id.to_string(),
            workspace_id: ws.id.to_string(),
            token,
        })
    }

    fn app(state: AppState) -> axum::Router {
        create_tasks_router().with_state(state)
    }

    async fn call(
        state: AppState,
        method: &str,
        uri: &str,
        token: &str,
        body: Option<Value>,
    ) -> anyhow::Result<(StatusCode, Value)> {
        let mut b = Request::builder()
            .method(method)
            .uri(uri)
            .header("Authorization", format!("Bearer {token}"));
        if body.is_some() {
            b = b.header("Content-Type", "application/json");
        }
        let payload = body.map_or_else(Body::empty, |v| Body::from(v.to_string()));
        let resp = app(state).oneshot(b.body(payload)?).await?;
        let status = resp.status();
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        Ok((status, json))
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 044)"]
    async fn db_task_create_and_get_roundtrip() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let (status, created) = call(state.clone(), "POST", "/", &seed.token,
            Some(json!({"title": "Contract task", "workspace_id": seed.workspace_id, "assignee_id": seed.user_id}))).await?;
        anyhow::ensure!(status == StatusCode::CREATED, "create {status}: {created}");
        let id = created["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("no id: {created}"))?;
        let (status, got) = call(state, "GET", &format!("/{id}"), &seed.token, None).await?;
        anyhow::ensure!(status == StatusCode::OK, "get {status}: {got}");
        anyhow::ensure!(got["title"] == "Contract task", "title mismatch: {got}");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 044)"]
    async fn db_task_patch_status_and_assignee_tristate() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let (_, created) = call(state.clone(), "POST", "/", &seed.token,
            Some(json!({"title": "Patch task", "workspace_id": seed.workspace_id, "assignee_id": seed.user_id}))).await?;
        let id = created["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("no id"))?;
        let (status, patched) = call(
            state.clone(),
            "PATCH",
            &format!("/{id}"),
            &seed.token,
            Some(json!({"status": "InProgress"})),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "patch {status}: {patched}");
        anyhow::ensure!(
            patched["status"] == "InProgress",
            "status not applied: {patched}"
        );
        // tri-state: null unassigns
        let (status, unassigned) = call(
            state,
            "PATCH",
            &format!("/{id}"),
            &seed.token,
            Some(json!({"assignee_id": null})),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::OK, "unassign {status}: {unassigned}");
        anyhow::ensure!(
            unassigned["assignee_id"].is_null(),
            "tri-state null did not clear assignee: {unassigned}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 044)"]
    async fn db_task_soft_delete_hides_from_get() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let (_, created) = call(state.clone(), "POST", "/", &seed.token,
            Some(json!({"title": "Delete task", "workspace_id": seed.workspace_id, "assignee_id": seed.user_id}))).await?;
        let id = created["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("no id"))?;
        let (status, _) = call(
            state.clone(),
            "DELETE",
            &format!("/{id}"),
            &seed.token,
            None,
        )
        .await?;
        anyhow::ensure!(status == StatusCode::NO_CONTENT, "delete {status}");
        let (status, _) = call(state, "GET", &format!("/{id}"), &seed.token, None).await?;
        anyhow::ensure!(
            status == StatusCode::NOT_FOUND,
            "deleted task still visible: {status}"
        );
        Ok(())
    }
}
