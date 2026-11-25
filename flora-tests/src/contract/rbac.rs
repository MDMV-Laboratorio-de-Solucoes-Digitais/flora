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
        user_id: uuid::Uuid,
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
        Ok(Seed {
            org,
            role,
            user_id: user.id,
            token,
        })
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

    async fn mint_org_session(
        pool: &PgPool,
        config: &Config,
        user: &User,
        org_id: uuid::Uuid,
        prefix: &str,
    ) -> anyhow::Result<String> {
        let jti = format!("jti-{prefix}-{}", uuid::Uuid::now_v7());
        let now = Utc::now();
        let _session = PgSessionRepository::new(pool.clone())
            .create(Session {
                id: uuid::Uuid::now_v7(),
                user_id: user.id,
                organization_id: org_id,
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
        Ok(encode_token(
            &FloraClaims::new(
                user.id,
                jti,
                org_id,
                &user.email,
                config.app.session_ttl_secs,
            ),
            &config.app.jwt_secret,
        ))
    }

    async fn seed_additional_member(
        state: &AppState,
        config: &Config,
        org_id: uuid::Uuid,
        role: Role,
        prefix: &str,
    ) -> anyhow::Result<(User, String)> {
        let pool = (*state.db_pool).clone();
        let uniq = uuid::Uuid::now_v7().simple().to_string();
        let user = PgUserRepository::new(pool.clone())
            .create(User::new(
                &format!("{prefix}-{uniq}@example.test"),
                "Additional Member",
            ))
            .await?;
        let _membership = PgMembershipRepository::new(pool.clone())
            .create(Membership::new(user.id, org_id, role.id))
            .await?;
        let token = mint_org_session(&pool, config, &user, org_id, prefix).await?;
        Ok((user, token))
    }

    async fn create_org_role(
        repo: &PgRoleRepository,
        org_id: uuid::Uuid,
        name: &str,
        perms: &[Permission],
    ) -> anyhow::Result<Role> {
        let mut role = Role::new(org_id, name);
        for permission in perms {
            role.add_permission(*permission);
        }
        Ok(repo.create(role).await?)
    }

    fn assert_member_list_sanitized(
        members: &Value,
        other_user_id: uuid::Uuid,
    ) -> anyhow::Result<()> {
        let listed = members
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("expected array: {members}"))?;
        anyhow::ensure!(listed.len() == 2, "expected 2 org members, got {listed:?}");
        anyhow::ensure!(
            listed.iter().all(|m| {
                m.as_object().is_some_and(|o| {
                    !o.contains_key("password_hash")
                        && !o.contains_key("oidc_subject")
                        && !o.contains_key("profile")
                        && o["user_id"] != serde_json::json!(other_user_id)
                })
            }),
            "member response leaked sensitive fields or other tenant: {listed:?}"
        );
        Ok(())
    }

    async fn assert_add_member_validation_edges(
        state: &AppState,
        admin_token: &str,
        valid_role_id: uuid::Uuid,
        other_org_role_id: uuid::Uuid,
        active_email: &str,
    ) -> anyhow::Result<()> {
        let pool = (*state.db_pool).clone();
        let (status, _) = request(
            state,
            "POST",
            "/members",
            Some(admin_token),
            Some(serde_json::json!({ "email": "not-an-email", "role_id": valid_role_id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::UNPROCESSABLE_ENTITY);

        let uniq = uuid::Uuid::now_v7().simple().to_string();
        let inactive = PgUserRepository::new(pool.clone())
            .create(User::new(
                &format!("inactive-{uniq}@example.test"),
                "Inactive",
            ))
            .await?;
        let _ = sqlx::query("UPDATE users SET is_active = false WHERE id = $1")
            .bind(inactive.id)
            .execute(&pool)
            .await?;
        let (status, _) = request(
            state,
            "POST",
            "/members",
            Some(admin_token),
            Some(serde_json::json!({ "email": inactive.email, "role_id": valid_role_id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::NOT_FOUND);

        let (status, _) = request(
            state,
            "POST",
            "/members",
            Some(admin_token),
            Some(serde_json::json!({ "email": active_email, "role_id": other_org_role_id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::NOT_FOUND);

        let (status, _) = request(
            state,
            "POST",
            &format!("/{valid_role_id}/assign"),
            Some(admin_token),
            Some(serde_json::json!({ "user_id": inactive.id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::NOT_FOUND);
        Ok(())
    }

    async fn assert_assign_and_revoke_lifecycle(
        state: &AppState,
        admin: &Seed,
        target_user_id: uuid::Uuid,
        target_token: &str,
        role_ids: (uuid::Uuid, uuid::Uuid, uuid::Uuid),
    ) -> anyhow::Result<()> {
        let (reader_role_id, editor_role_id, other_org_role_id) = role_ids;
        let org_id = admin.org.id;
        let admin_token = &admin.token;
        let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
        let initial_membership = membership_repo
            .find_by_user_and_organization(target_user_id, org_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("membership missing after add"))?;
        let (status, _) = request(
            state,
            "POST",
            &format!("/{other_org_role_id}/assign"),
            Some(admin_token),
            Some(serde_json::json!({ "user_id": target_user_id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::NOT_FOUND);

        let (status, _) = request(
            state,
            "POST",
            &format!("/{editor_role_id}/assign"),
            Some(admin_token),
            Some(serde_json::json!({ "user_id": target_user_id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::NO_CONTENT);
        let updated = membership_repo
            .find_by_user_and_organization(target_user_id, org_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("membership disappeared after assign"))?;
        anyhow::ensure!(updated.role_id == editor_role_id);
        anyhow::ensure!(updated.joined_at == initial_membership.joined_at);

        let (status, _) = request(
            state,
            "POST",
            &format!("/{reader_role_id}/revoke"),
            Some(admin_token),
            Some(serde_json::json!({ "user_id": target_user_id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CONFLICT);

        let (status, _) = request(
            state,
            "POST",
            &format!("/{editor_role_id}/revoke"),
            Some(admin_token),
            Some(serde_json::json!({ "user_id": target_user_id })),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::NO_CONTENT);
        anyhow::ensure!(
            membership_repo
                .find_by_user_and_organization(target_user_id, org_id)
                .await?
                .is_none()
        );
        let (status, _) = request(state, "GET", "/members", Some(target_token), None).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "revoked member token must be rejected, got {status}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 122)"]
    async fn db_rbac_members_list_add_assign_and_revoke_lifecycle() -> anyhow::Result<()> {
        let (state, config) = db_state().await?;
        let pool = (*state.db_pool).clone();
        let admin = seed_member(
            &state,
            &config,
            &[Permission::OrgRead, Permission::RoleWrite],
        )
        .await?;
        let role_repo = PgRoleRepository::new(pool.clone());
        let reader_role = create_org_role(
            &role_repo,
            admin.org.id,
            "ReaderRole",
            &[Permission::OrgRead],
        )
        .await?;
        let editor_role = create_org_role(
            &role_repo,
            admin.org.id,
            "EditorRole",
            &[Permission::OrgRead, Permission::TaskWrite],
        )
        .await?;
        let (_, reader_token) =
            seed_additional_member(&state, &config, admin.org.id, reader_role.clone(), "reader")
                .await?;
        let uniq = uuid::Uuid::now_v7().simple().to_string();
        let target_user = PgUserRepository::new(pool.clone())
            .create(User::new(
                &format!("target-{uniq}@example.test"),
                "Target Member",
            ))
            .await?;
        let other_org = seed_member(&state, &config, &[Permission::OrgAdmin]).await?;

        let (status, _) = request(&state, "GET", "/members", None, None).await?;
        anyhow::ensure!(status == StatusCode::UNAUTHORIZED);
        let (status, members) =
            request(&state, "GET", "/members", Some(&reader_token), None).await?;
        anyhow::ensure!(status == StatusCode::OK);
        assert_member_list_sanitized(&members, other_org.user_id)?;

        assert_add_member_validation_edges(
            &state,
            &admin.token,
            reader_role.id,
            other_org.role.id,
            &target_user.email,
        )
        .await?;

        let add_payload = serde_json::json!({
            "email": target_user.email.to_uppercase(),
            "role_id": reader_role.id,
        });
        let (status, _) = request(
            &state,
            "POST",
            "/members",
            Some(&reader_token),
            Some(add_payload.clone()),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::FORBIDDEN);
        let (status, added) = request(
            &state,
            "POST",
            "/members",
            Some(&admin.token),
            Some(add_payload.clone()),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::CREATED
                && added["user_id"] == serde_json::json!(target_user.id)
                && added["role_id"] == serde_json::json!(reader_role.id),
            "unexpected add member response {status}: {added}"
        );
        let (status, _) = request(
            &state,
            "POST",
            "/members",
            Some(&admin.token),
            Some(add_payload),
        )
        .await?;
        anyhow::ensure!(status == StatusCode::CONFLICT);

        let target_token =
            mint_org_session(&pool, &config, &target_user, admin.org.id, "target").await?;
        let (status, _) = request(&state, "GET", "/members", Some(&target_token), None).await?;
        anyhow::ensure!(status == StatusCode::OK);

        assert_assign_and_revoke_lifecycle(
            &state,
            &admin,
            target_user.id,
            &target_token,
            (reader_role.id, editor_role.id, other_org.role.id),
        )
        .await
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 122)"]
    async fn db_rbac_members_prevents_self_demotion_and_self_revocation() -> anyhow::Result<()> {
        let (state, config) = db_state().await?;
        let admin = seed_member(
            &state,
            &config,
            &[Permission::OrgAdmin, Permission::RoleWrite],
        )
        .await?;
        let pool = (*state.db_pool).clone();
        let viewer_role = PgRoleRepository::new(pool.clone())
            .create({
                let mut r = Role::new(admin.org.id, "ViewerSelfCheck");
                r.add_permission(Permission::OrgRead);
                r
            })
            .await?;

        let (status, body) = request(
            &state,
            "POST",
            &format!("/{}/assign", viewer_role.id),
            Some(&admin.token),
            Some(serde_json::json!({ "user_id": admin.user_id })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::FORBIDDEN,
            "self-demotion via assign must return 403, got {status}: {body}"
        );

        let (status, body) = request(
            &state,
            "POST",
            &format!("/{}/revoke", admin.role.id),
            Some(&admin.token),
            Some(serde_json::json!({ "user_id": admin.user_id })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::FORBIDDEN,
            "self-revocation via revoke must return 403, got {status}: {body}"
        );

        let (status, _) = request(
            &state,
            "POST",
            "/members",
            Some(&admin.token),
            Some(serde_json::json!({
                "email": "someone@example.test",
                "role_id": viewer_role.id,
                "extra": true
            })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::UNPROCESSABLE_ENTITY,
            "unknown field in AddMemberRequest must be rejected, got {status}"
        );

        let (status, _) = request(
            &state,
            "POST",
            &format!("/{}/assign", viewer_role.id),
            Some(&admin.token),
            Some(serde_json::json!({
                "user_id": uuid::Uuid::now_v7(),
                "extra": true
            })),
        )
        .await?;
        anyhow::ensure!(
            status == StatusCode::UNPROCESSABLE_ENTITY,
            "unknown field in UserRoleRequest must be rejected, got {status}"
        );

        let membership = PgMembershipRepository::new(pool)
            .find_by_user_and_organization(admin.user_id, admin.org.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("admin membership must remain intact"))?;
        anyhow::ensure!(membership.role_id == admin.role.id);
        Ok(())
    }
}
