//! DB-gated contract tests for the role management endpoints (cycle 070).
//!
//! These replace the previous vacuous `Ok(())` stub with real assertions on the
//! Axum router backed by a disposable PostgreSQL 18 database. They run only
//! with `FLORA_DATABASE__POSTGRES_URL` set and `#[ignore]` (the `db-tests` CI leg).

#[cfg(test)]
mod db {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::{Duration, Utc};
    use flora_api::routes::rbac::create_rbac_router;
    use flora_api::state::AppState;
    use flora_core::config::Config;
    use flora_core::migrations::run_migrations;
    use flora_core::models::{Membership, Organization, Permission, Role, Session, User};
    use flora_core::repositories::{
        PgMembershipRepository, PgOrganizationRepository, PgRoleRepository, PgSessionRepository,
        PgUserRepository,
    };
    use flora_core::traits::{
        MembershipRepository, OrganizationRepository, RoleRepository, SessionRepository,
        UserRepository,
    };
    use flora_core::utils::jwt::{FloraClaims, encode_token};
    use serde_json::Value;
    use sqlx::PgPool;
    use tower::ServiceExt;

    use super::super::auth::make_test_state;

    struct Seed {
        org: Organization,
        role: Role,
        token: String,
    }

    async fn db_state() -> anyhow::Result<(AppState, Config)> {
        let url = std::env::var("FLORA_DATABASE__POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("DATABASE_URL").ok())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "FLORA_DATABASE__POSTGRES_URL/DATABASE_URL required for DB contract tests"
                )
            })?;
        let pool = PgPool::connect(&url).await?;
        run_migrations(&pool).await?;
        let config = Config::default();
        Ok((make_test_state(pool, &config), config))
    }

    async fn seed_member(
        state: &AppState,
        config: &Config,
        permissions: &[Permission],
    ) -> anyhow::Result<Seed> {
        let pool = (*state.db_pool).clone();
        let uniq = uuid::Uuid::now_v7().simple().to_string();
        let user = PgUserRepository::new(pool.clone())
            .create(User::new(
                &format!("rbac-{uniq}@example.test"),
                "RBAC Test User",
            ))
            .await?;
        let org = PgOrganizationRepository::new(pool.clone())
            .create(Organization::new(
                &format!("RbacOrg {uniq}"),
                &format!("rbacorg-{uniq}"),
            ))
            .await?;
        let mut role = Role::new(org.id, "ContractRole");
        for permission in permissions {
            role.add_permission(*permission);
        }
        let role = PgRoleRepository::new(pool.clone()).create(role).await?;
        let _membership = PgMembershipRepository::new(pool.clone())
            .create(Membership::new(user.id, org.id, role.id))
            .await?;

        let jti = format!("jti-{}", uuid::Uuid::now_v7());
        let now = Utc::now();
        let _session = PgSessionRepository::new(pool)
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
        Ok(Seed { org, role, token })
    }

    async fn request(
        state: &AppState,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: Option<Value>,
    ) -> anyhow::Result<(StatusCode, Value)> {
        let mut builder = Request::builder().method(method).uri(path);
        if let Some(token) = token {
            builder = builder.header("Authorization", format!("Bearer {token}"));
        }
        if body.is_some() {
            builder = builder.header("Content-Type", "application/json");
        }
        let payload = match body {
            Some(value) => Body::from(serde_json::to_vec(&value)?),
            None => Body::empty(),
        };
        let response = create_rbac_router()
            .with_state(state.clone())
            .oneshot(builder.body(payload)?)
            .await?;
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
        let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        Ok((status, json))
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 070)"]
    async fn db_rbac_list_scoped_to_session_org() -> anyhow::Result<()> {
        let (state, config) = db_state().await?;
        let admin = seed_member(
            &state,
            &config,
            &[Permission::OrgRead, Permission::RoleWrite],
        )
        .await?;
        let other = seed_member(&state, &config, &[Permission::OrgRead]).await?;

        let (status, body) = request(&state, "GET", "/", Some(&admin.token), None).await?;
        anyhow::ensure!(status == StatusCode::OK, "list status {status}: {body}");
        let roles = body
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("not an array: {body}"))?;
        anyhow::ensure!(
            roles.iter().any(|r| r["id"] == admin.role.id.to_string()),
            "own role missing from list: {body}"
        );
        anyhow::ensure!(
            !roles.iter().any(|r| r["id"] == other.role.id.to_string()),
            "CROSS-ORG ROLE LEAK: {body}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 070)"]
    async fn db_rbac_create_requires_role_write_and_uses_session_org() -> anyhow::Result<()> {
        let (state, config) = db_state().await?;
        let admin = seed_member(
            &state,
            &config,
            &[Permission::OrgRead, Permission::RoleWrite],
        )
        .await?;
        let reader = seed_member(&state, &config, &[Permission::OrgRead]).await?;
        let payload = serde_json::json!({
            "name": "Contract Auditor",
            "description": "created by contract test",
            "permissions": ["org:read"],
        });

        let (status, body) = request(
            &state,
            "POST",
            "/",
            Some(&reader.token),
            Some(payload.clone()),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::FORBIDDEN,
            "reader must be denied: {status} {body}"
        );

        let (status, body) =
            request(&state, "POST", "/", Some(&admin.token), Some(payload)).await?;
        anyhow::ensure!(
            status == StatusCode::CREATED,
            "create status {status}: {body}"
        );
        let created_id: uuid::Uuid = body["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing id: {body}"))?
            .parse()?;
        let stored = PgRoleRepository::new((*state.db_pool).clone())
            .find_by_id(created_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("created role not persisted"))?;
        anyhow::ensure!(
            stored.organization_id == admin.org.id,
            "role must be scoped to the session organization"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 070)"]
    async fn db_rbac_builtin_immutable_custom_patchable() -> anyhow::Result<()> {
        let (state, config) = db_state().await?;
        let admin = seed_member(
            &state,
            &config,
            &[Permission::OrgRead, Permission::RoleWrite],
        )
        .await?;
        let repo = PgRoleRepository::new((*state.db_pool).clone());

        let mut builtin = Role::new(admin.org.id, "BuiltinContractRole");
        builtin.is_builtin = true;
        let builtin = repo.create(builtin).await?;
        let custom = Role::new(admin.org.id, "CustomContractRole");
        let custom = repo.create(custom).await?;

        let rename = serde_json::json!({"name": "Renamed"});
        let (status, body) = request(
            &state,
            "PATCH",
            &format!("/{}", builtin.id),
            Some(&admin.token),
            Some(rename.clone()),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::FORBIDDEN,
            "built-in must be immutable: {status} {body}"
        );

        let (status, body) = request(
            &state,
            "PATCH",
            &format!("/{}", custom.id),
            Some(&admin.token),
            Some(rename),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::OK,
            "custom patch status {status}: {body}"
        );
        anyhow::ensure!(body["name"] == "Renamed", "name not updated: {body}");
        Ok(())
    }
}
