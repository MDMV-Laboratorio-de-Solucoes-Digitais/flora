//! DB-gated contract tests for workspace management endpoints (cycle 044).
//!
//! These replace the previous `Ok(())` stubs with real assertions on the
//! Axum router backed by a disposable PostgreSQL 18. They run only with
//! `FLORA_DATABASE__POSTGRES_URL` set and `#[ignore]` (the `db-tests` CI leg).

#[cfg(test)]
mod db {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::{Duration, Utc};
    use flora_api::routes::workspace::create_workspace_router;
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
    use serde_json::Value;
    use sqlx::PgPool;
    use tower::ServiceExt;

    use super::super::auth::make_test_state;

    struct Seed {
        org: Organization,
        token: String,
    }

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
        run_migrations(&pool).await?;
        let config = Config::default();
        Ok(Some((make_test_state(pool, &config), config)))
    }

    async fn seed_member(state: &AppState, config: &Config) -> anyhow::Result<Seed> {
        let uniq = uuid::Uuid::now_v7().simple().to_string();
        let user = PgUserRepository::new((*state.db_pool).clone())
            .create(User::new(
                &format!("ws-{uniq}@example.test"),
                "WS Test User",
            ))
            .await?;
        let org = PgOrganizationRepository::new((*state.db_pool).clone())
            .create(Organization::new(
                &format!("WsOrg {uniq}"),
                &format!("wsorg-{uniq}"),
            ))
            .await?;
        let role = PgRoleRepository::new((*state.db_pool).clone())
            .create(Role::new(org.id, "Owner"))
            .await?;
        let _membership = PgMembershipRepository::new((*state.db_pool).clone())
            .create(Membership::new(user.id, org.id, role.id))
            .await?;
        let jti = format!("jti-{}", uuid::Uuid::now_v7());
        let now = Utc::now();
        let _session = PgSessionRepository::new((*state.db_pool).clone())
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
        Ok(Seed { org, token })
    }

    fn ws_app(state: AppState) -> axum::Router {
        create_workspace_router().with_state(state)
    }

    async fn json_body(resp: axum::response::Response) -> Value {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    }

    async fn authed_get(
        state: AppState,
        path: &str,
        token: &str,
    ) -> anyhow::Result<(StatusCode, Value)> {
        let resp = ws_app(state)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(path)
                    .header("Authorization", format!("Bearer {token}"))
                    .body(Body::empty())?,
            )
            .await?;
        let status = resp.status();
        Ok((status, json_body(resp).await))
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 044)"]
    async fn db_workspace_list_returns_seeded_workspace() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed = seed_member(&state, &config).await?;
        let repo = PgWorkspaceRepository::new((*state.db_pool).clone());
        let ws = repo
            .create(Workspace::new(seed.org.id, "Contract WS", None))
            .await?;

        let (status, body) = authed_get(state, "/", &seed.token).await?;
        anyhow::ensure!(status == StatusCode::OK, "list status {status}: {body}");
        let arr = body
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("not an array: {body}"))?;
        anyhow::ensure!(
            arr.iter().any(|w| w["id"] == ws.id.to_string()),
            "workspace missing: {body}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 064)"]
    async fn db_n3_cross_org_role_lookup_rejected() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed_a = seed_member(&state, &config).await?;
        let seed_b = seed_member(&state, &config).await?;
        let repo = PgRoleRepository::new((*state.db_pool).clone());
        let role_b = repo
            .find_by_organization_id(seed_b.org.id)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("no role seeded for org B"))?;

        let cross = repo
            .find_by_id_and_organization(role_b.id, seed_a.org.id)
            .await?;
        anyhow::ensure!(cross.is_none(), "CROSS-ORG ROLE LEAK");

        let own = repo
            .find_by_id_and_organization(role_b.id, seed_b.org.id)
            .await?;
        anyhow::ensure!(own.is_some(), "own role must still resolve");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 044)"]
    async fn db_workspace_isolation_between_orgs() -> anyhow::Result<()> {
        let Some((state, config)) = db_state().await? else {
            return Ok(());
        };
        let seed_a = seed_member(&state, &config).await?;
        let seed_b = seed_member(&state, &config).await?;
        let repo = PgWorkspaceRepository::new((*state.db_pool).clone());
        let ws_a = repo
            .create(Workspace::new(seed_a.org.id, "WS-A", None))
            .await?;
        let ws_b = repo
            .create(Workspace::new(seed_b.org.id, "WS-B", None))
            .await?;

        let (status, body) = authed_get(state, "/", &seed_a.token).await?;
        anyhow::ensure!(status == StatusCode::OK, "list status {status}");
        let arr = body
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("not an array: {body}"))?;
        anyhow::ensure!(
            arr.iter().any(|w| w["id"] == ws_a.id.to_string()),
            "own workspace missing"
        );
        anyhow::ensure!(
            !arr.iter().any(|w| w["id"] == ws_b.id.to_string()),
            "CROSS-ORG LEAK: {body}"
        );
        Ok(())
    }
}
