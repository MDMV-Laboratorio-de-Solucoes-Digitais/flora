//! DB-gated contract tests for `POST /api/v1/hitl/receipts` (cycle 120 / BF-03).

#[cfg(test)]
mod db {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::{Duration, Utc};
    use flora_api::adapters::HitlReceiptAdapter;
    use flora_api::routes::create_router;
    use flora_api::state::AppState;
    use flora_core::config::Config;
    use flora_core::migrations::run_migrations;
    use flora_core::models::{
        HitlDecision, HitlReceipt, Membership, Organization, Permission, Role, Session, User,
    };
    use flora_core::repositories::{
        PgMembershipRepository, PgOrganizationRepository, PgRoleRepository, PgSessionRepository,
        PgUserRepository,
    };
    use flora_core::traits::{
        HitlReceiptPort, MembershipRepository, OrganizationRepository, RoleRepository,
        SessionRepository, UserRepository,
    };
    use flora_core::utils::jwt::{FloraClaims, encode_token};
    use serde_json::{Value, json};
    use sqlx::PgPool;
    use tower::ServiceExt;

    use super::super::auth::make_test_state;

    const TEST_HITL_SECRET: &str = "d15-contract-test-secret-0123456789abcdef";

    struct Seed {
        user_id: uuid::Uuid,
        token: String,
    }

    async fn db_state(hmac_secret: &str) -> anyhow::Result<(AppState, Config)> {
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
        let mut config = Config::default();
        hmac_secret.clone_into(&mut config.hitl.hmac_secret);
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
                &format!("hitl-{uniq}@example.test"),
                "HITL Operator Test",
            ))
            .await?;
        let org = PgOrganizationRepository::new(pool.clone())
            .create(Organization::new(
                &format!("HitlOrg {uniq}"),
                &format!("hitlorg-{uniq}"),
            ))
            .await?;
        let mut role = Role::new(org.id, "HitlContractRole");
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
            user_id: user.id,
            token,
        })
    }

    fn valid_request_body() -> Value {
        json!({
            "work_order_id": "wo-120-001",
            "ativo_id": "ativo-120-001",
            "station": "S3_VERIFICATION",
            "artifacts_hash": "a".repeat(64),
            "operator_approved": true,
            "pii_redacted": true,
            "real_external_send": false,
            "next_stage": "s4_monetization"
        })
    }

    async fn post_hitl(
        state: &AppState,
        token: Option<&str>,
        body: Value,
    ) -> anyhow::Result<(StatusCode, Value, String)> {
        let mut builder = Request::builder()
            .method("POST")
            .uri("/api/v1/hitl/receipts")
            .header("Content-Type", "application/json");
        if let Some(token) = token {
            builder = builder.header("Authorization", format!("Bearer {token}"));
        }
        let response = create_router(state.clone())
            .oneshot(builder.body(Body::from(serde_json::to_vec(&body)?))?)
            .await?;
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
        let raw = String::from_utf8_lossy(&bytes).into_owned();
        let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        Ok((status, json, raw))
    }

    #[tokio::test]
    #[ignore = "requires Postgres (FLORA_DATABASE__POSTGRES_URL)"]
    async fn db_hitl_emit_receipt_succeeds_for_org_admin_and_binds_server_operator_id()
    -> anyhow::Result<()> {
        let (state, config) = db_state(TEST_HITL_SECRET).await?;
        let admin = seed_member(&state, &config, &[Permission::OrgAdmin]).await?;

        let (status, body, _) = post_hitl(&state, Some(&admin.token), valid_request_body()).await?;
        anyhow::ensure!(
            status == StatusCode::CREATED,
            "expected 201 Created, got {status}: {body}"
        );

        let receipt: HitlReceipt = serde_json::from_value(body)?;
        anyhow::ensure!(receipt.producer_system == "flora");
        anyhow::ensure!(receipt.status == "verified");
        anyhow::ensure!(receipt.work_order_id == "wo-120-001");
        anyhow::ensure!(receipt.ativo_id == "ativo-120-001");
        anyhow::ensure!(receipt.station == "S3_VERIFICATION");
        anyhow::ensure!(
            receipt.metrics_payload["operator_id"] == admin.user_id.to_string(),
            "operator_id must be derived from authenticated session"
        );

        let expected = HitlReceiptAdapter::new(TEST_HITL_SECRET).emit(&HitlDecision {
            work_order_id: "wo-120-001".to_owned(),
            ativo_id: "ativo-120-001".to_owned(),
            station: "S3_VERIFICATION".to_owned(),
            artifacts_hash: "a".repeat(64),
            operator_approved: true,
            operator_id: admin.user_id.to_string(),
            pii_redacted: true,
            real_external_send: false,
            next_stage: "s4_monetization".to_owned(),
        })?;
        anyhow::ensure!(receipt.signature_hmac == expected.signature_hmac);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres (FLORA_DATABASE__POSTGRES_URL)"]
    async fn db_hitl_emit_receipt_rejects_unauthenticated_non_admin_and_spoofed_operator_id()
    -> anyhow::Result<()> {
        let (state, config) = db_state(TEST_HITL_SECRET).await?;
        let member = seed_member(
            &state,
            &config,
            &[Permission::OrgRead, Permission::TaskWrite],
        )
        .await?;
        let admin = seed_member(&state, &config, &[Permission::OrgAdmin]).await?;

        let (unauth_status, unauth_body, _) = post_hitl(&state, None, valid_request_body()).await?;
        anyhow::ensure!(
            unauth_status == StatusCode::UNAUTHORIZED,
            "expected 401, got {unauth_status}: {unauth_body}"
        );

        let (member_status, member_body, _) =
            post_hitl(&state, Some(&member.token), valid_request_body()).await?;
        anyhow::ensure!(
            member_status == StatusCode::FORBIDDEN,
            "expected 403, got {member_status}: {member_body}"
        );

        let mut spoofed = valid_request_body();
        if let Some(obj) = spoofed.as_object_mut() {
            let _ = obj.insert("operator_id".to_owned(), json!("forged-op"));
        }
        let (spoof_status, _, spoof_raw) = post_hitl(&state, Some(&admin.token), spoofed).await?;
        anyhow::ensure!(
            spoof_status == StatusCode::UNPROCESSABLE_ENTITY,
            "expected 422 for spoofed operator_id, got {spoof_status}: {spoof_raw}"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres (FLORA_DATABASE__POSTGRES_URL)"]
    async fn db_hitl_emit_receipt_fails_closed_on_invalid_decision_and_missing_secret()
    -> anyhow::Result<()> {
        let (state, config) = db_state("").await?;
        let admin = seed_member(&state, &config, &[Permission::OrgAdmin]).await?;

        for (field, bad_val) in [
            ("pii_redacted", json!(false)),
            ("real_external_send", json!(true)),
            ("next_stage", json!("S4_MONETIZATION")),
        ] {
            let mut invalid = valid_request_body();
            if let Some(obj) = invalid.as_object_mut() {
                let _ = obj.insert(field.to_owned(), bad_val);
            }
            let (status, body, _) = post_hitl(&state, Some(&admin.token), invalid).await?;
            anyhow::ensure!(
                status == StatusCode::UNPROCESSABLE_ENTITY,
                "expected 422 for {field}, got {status}: {body}"
            );
        }

        let (unavail_status, unavail_body, unavail_raw) =
            post_hitl(&state, Some(&admin.token), valid_request_body()).await?;
        anyhow::ensure!(
            unavail_status == StatusCode::SERVICE_UNAVAILABLE,
            "expected 503 when hmac secret missing, got {unavail_status}: {unavail_raw}"
        );
        anyhow::ensure!(unavail_body["error"] == "service_unavailable");
        anyhow::ensure!(
            !unavail_raw.contains("FLORA_HITL__HMAC_SECRET"),
            "503 response leaked secret env var name: {unavail_raw}"
        );
        Ok(())
    }
}
