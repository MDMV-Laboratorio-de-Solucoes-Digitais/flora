//! Contract tests for the one-time `POST /api/v1/bootstrap` flow.
//!
//! Each test uses a fresh temporary database so the "empty installation"
//! precondition is deterministic and independent of other DB contract tests.

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::body::{Body, to_bytes};
    use axum::http::{Method, Request, StatusCode};
    use axum::response::Response;
    use flora_api::routes::bootstrap::create_bootstrap_router;
    use flora_api::state::AppState;
    use flora_core::config::Config;
    use flora_core::migrations::run_migrations;
    use flora_core::models::Organization;
    use flora_core::repositories::{PgOrganizationRepository, PgSessionRepository};
    use flora_core::traits::{OrganizationRepository, SessionRepository};
    use flora_core::utils::jwt::decode_token;
    use serde_json::{Value, json};
    use sqlx::PgPool;
    use std::sync::Arc;
    use tower::ServiceExt;
    use url::Url;
    use uuid::Uuid;

    struct TestDb {
        url: String,
        name: String,
        admin: PgPool,
    }

    impl TestDb {
        async fn create() -> anyhow::Result<Self> {
            let base = std::env::var("FLORA_DATABASE__POSTGRES_URL")
                .ok()
                .or_else(|| std::env::var("DATABASE_URL").ok())
                .ok_or_else(|| anyhow::anyhow!("DATABASE_URL required for bootstrap tests"))?;
            let name = format!("flora_bootstrap_{}", Uuid::now_v7().simple());

            let mut admin_url = Url::parse(&base)?;
            admin_url.set_path("/postgres");
            let admin = PgPool::connect(admin_url.as_str()).await?;
            let _ = sqlx::raw_sql(sqlx::AssertSqlSafe(format!("CREATE DATABASE {name}")))
                .execute(&admin)
                .await?;

            let mut db_url = Url::parse(&base)?;
            db_url.set_path(&format!("/{name}"));
            let url = db_url.to_string();
            let pool = PgPool::connect(&url).await?;
            run_migrations(&pool).await?;
            Ok(Self { url, name, admin })
        }

        async fn state(&self) -> anyhow::Result<(AppState, Config)> {
            let pool = PgPool::connect(&self.url).await?;
            let config = Config::default();
            let state = crate::contract::auth::make_test_state(pool, &config);
            Ok((state, config))
        }

        async fn cleanup(self) -> anyhow::Result<()> {
            let _ = sqlx::raw_sql(sqlx::AssertSqlSafe(format!(
                "DROP DATABASE IF EXISTS {} WITH (FORCE)",
                self.name
            )))
            .execute(&self.admin)
            .await;
            Ok(())
        }
    }

    fn router(state: AppState) -> Router {
        Router::new()
            .nest("/api/v1", create_bootstrap_router())
            .with_state(state)
    }

    fn body(name: &str, email: &str) -> Value {
        json!({ "organization_name": name, "admin_email": email })
    }

    async fn post(router: Router, payload: Value) -> anyhow::Result<Response> {
        Ok(router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/v1/bootstrap")
                    .header("content-type", "application/json")
                    .header("user-agent", "contract-test")
                    .body(Body::from(serde_json::to_vec(&payload)?))?,
            )
            .await?)
    }

    async fn json_body(response: Response) -> anyhow::Result<Value> {
        let bytes = to_bytes(response.into_body(), usize::MAX).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn count(db: &TestDb, table: &str) -> anyhow::Result<i64> {
        let pool = PgPool::connect(&db.url).await?;
        match table {
            "organizations" => Ok(sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM organizations \
                 WHERE id <> '00000000-0000-0000-0000-000000000000'",
            )
            .fetch_one(&pool)
            .await?),
            "users" => Ok(sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
                .fetch_one(&pool)
                .await?),
            "roles" => Ok(sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM roles")
                .fetch_one(&pool)
                .await?),
            "memberships" => Ok(
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM memberships")
                    .fetch_one(&pool)
                    .await?,
            ),
            "workspaces" => Ok(
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM workspaces")
                    .fetch_one(&pool)
                    .await?,
            ),
            "sessions" => Ok(
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sessions")
                    .fetch_one(&pool)
                    .await?,
            ),
            other => anyhow::bail!("unknown table: {other}"),
        }
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]
    async fn db_bootstrap_creates_org_admin_role_membership_workspace_and_session()
    -> anyhow::Result<()> {
        let db = TestDb::create().await?;
        let (state, config) = db.state().await?;

        let response = post(
            router(state),
            body("Minha Organização", "Admin@Example.Test"),
        )
        .await?;
        assert_eq!(response.status(), StatusCode::CREATED);
        let json = json_body(response).await?;

        assert_eq!(count(&db, "organizations").await?, 1);
        assert_eq!(count(&db, "users").await?, 1);
        assert_eq!(count(&db, "roles").await?, 1);
        assert_eq!(count(&db, "memberships").await?, 1);
        assert_eq!(count(&db, "workspaces").await?, 1);
        assert_eq!(count(&db, "sessions").await?, 1);

        let org_id = json["organization_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing organization_id"))?;
        let user_id = json["user_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing user_id"))?;
        let claims = decode_token(
            json["session_token"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("missing session_token"))?,
            &config.app.jwt_secret,
        )?;
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.organization_id, org_id);
        assert_eq!(claims.email, "admin@example.test");

        let pool = PgPool::connect(&db.url).await?;
        let session = PgSessionRepository::new(pool.clone())
            .find_by_jti(&claims.jti)
            .await?
            .ok_or_else(|| anyhow::anyhow!("session not found for jti"))?;
        assert!(session.is_active);
        assert_eq!(session.organization_id.to_string(), org_id);

        db.cleanup().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]
    async fn db_bootstrap_rejects_second_call_without_writes() -> anyhow::Result<()> {
        let db = TestDb::create().await?;
        let (state, config) = db.state().await?;
        let router = router(state);
        let config = Arc::new(config);

        let first = post(router.clone(), body("Primeira Org", "first@example.test")).await?;
        assert_eq!(first.status(), StatusCode::CREATED);

        let second = post(router, body("Segunda Org", "second@example.test")).await?;
        assert_eq!(second.status(), StatusCode::CONFLICT);
        let error = json_body(second).await?;
        assert_eq!(error["error"], "conflict");

        assert_eq!(count(&db, "organizations").await?, 1);
        assert_eq!(count(&db, "users").await?, 1);
        let _ = config;
        db.cleanup().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]
    async fn db_bootstrap_rejects_when_organization_exists() -> anyhow::Result<()> {
        let db = TestDb::create().await?;
        let (state, _config) = db.state().await?;
        let pool = (*state.db_pool).clone();
        let _ = PgOrganizationRepository::new(pool)
            .create(Organization::new("Existente", "existente"))
            .await?;

        let response = post(router(state), body("Nova Org", "novo@example.test")).await?;
        assert_eq!(response.status(), StatusCode::CONFLICT);
        assert_eq!(count(&db, "organizations").await?, 1);
        assert_eq!(count(&db, "users").await?, 0);
        db.cleanup().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]
    async fn db_bootstrap_concurrent_calls_create_single_organization() -> anyhow::Result<()> {
        let db = TestDb::create().await?;
        let (state, _config) = db.state().await?;
        let router = router(state);

        let (first, second) = tokio::join!(
            post(router.clone(), body("Concorrente A", "a@example.test")),
            post(router, body("Concorrente B", "b@example.test"))
        );
        let (first, second) = (first?, second?);
        let statuses = [first.status(), second.status()];
        assert_eq!(
            statuses
                .iter()
                .filter(|s| **s == StatusCode::CREATED)
                .count(),
            1
        );
        assert_eq!(
            statuses
                .iter()
                .filter(|s| **s == StatusCode::CONFLICT)
                .count(),
            1
        );
        assert_eq!(count(&db, "organizations").await?, 1);
        db.cleanup().await?;
        Ok(())
    }
}
