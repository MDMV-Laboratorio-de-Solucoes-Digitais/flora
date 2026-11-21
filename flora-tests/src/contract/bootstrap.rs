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
    use flora_core::models::{Organization, User};
    use flora_core::repositories::{
        PgOrganizationRepository, PgSessionRepository, PgUserRepository,
    };
    use flora_core::services::PasswordService;
    use flora_core::traits::{OrganizationRepository, SessionRepository, UserRepository};
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
        body_with_password(name, email, "SenhaAdministrador@123")
    }

    fn body_with_password(name: &str, email: &str, password: &str) -> Value {
        json!({
            "organization_name": name,
            "admin_email": email,
            "admin_password": password
        })
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

    fn lazy_state() -> anyhow::Result<AppState> {
        let pool = PgPool::connect_lazy("postgresql://localhost/flora_test")?;
        Ok(crate::contract::auth::make_test_state(
            pool,
            &Config::default(),
        ))
    }

    #[tokio::test]
    async fn bootstrap_rejects_missing_admin_password() -> anyhow::Result<()> {
        let payload = json!({
            "organization_name": "Minha Organização",
            "admin_email": "admin@example.test"
        });
        let response = post(router(lazy_state()?), payload).await?;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let error = json_body(response).await?;
        assert_eq!(error["field"], "admin_password");
        Ok(())
    }

    #[tokio::test]
    async fn bootstrap_rejects_admin_password_outside_length_bounds() -> anyhow::Result<()> {
        let state = lazy_state()?;
        let too_short = post(
            router(state.clone()),
            body_with_password("Org", "admin@example.test", "curta"),
        )
        .await?;
        assert_eq!(too_short.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(json_body(too_short).await?["field"], "admin_password");

        let too_long = post(
            router(state),
            body_with_password("Org", "admin@example.test", &"x".repeat(1025)),
        )
        .await?;
        assert_eq!(too_long.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(json_body(too_long).await?["field"], "admin_password");
        Ok(())
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

        let stored: (Option<String>, Option<chrono::DateTime<chrono::Utc>>) =
            sqlx::query_as("SELECT password_hash, password_updated_at FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_one(&pool)
                .await?;
        let hash = stored
            .0
            .ok_or_else(|| anyhow::anyhow!("bootstrap admin password hash was not stored"))?;
        assert!(hash.starts_with("$argon2id$v=19$m=19456,t=2,p=1$"));
        assert!(stored.1.is_some());
        assert!(PasswordService::shared().verify("SenhaAdministrador@123", &hash));

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
    #[tokio::test]
    #[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL"]
    async fn db_startup_password_applies_only_to_users_without_hash() -> anyhow::Result<()> {
        let db = TestDb::create().await?;
        let pool = PgPool::connect(&db.url).await?;
        let email = "luis.granja@mdmv.com.br";
        let user = PgUserRepository::new(pool.clone())
            .create(User::new(email, "Luis Granja"))
            .await?;
        assert!(user.password_hash.is_none());

        let mut config = Config::default();
        config.bootstrap = BootstrapConfig {
            admin_email: Some(email.to_owned()),
            admin_password: Some("SenhaInicial@12345".to_owned()),
        };

        let _state = AppState::new(pool.clone(), config.clone()).await?;
        let repo = PgUserRepository::new(pool.clone());
        let hash = repo
            .find_by_email(email)
            .await?
            .and_then(|found| found.password_hash)
            .ok_or_else(|| anyhow::anyhow!("startup password was not applied"))?;
        assert!(hash.starts_with("$argon2id$v=19$m=19456,t=2,p=1$"));
        assert!(PasswordService::shared().verify("SenhaInicial@12345", &hash));

        let mut second_startup = config.clone();
        second_startup.bootstrap.admin_password = Some("SenhaNova@12345".to_owned());
        let _state = AppState::new(pool.clone(), second_startup).await?;
        let unchanged = repo
            .find_by_email(email)
            .await?
            .and_then(|found| found.password_hash)
            .ok_or_else(|| anyhow::anyhow!("stored password hash disappeared"))?;
        assert_eq!(unchanged, hash);

        let mut missing_user = config;
        missing_user.bootstrap.admin_email = Some("ninguem@exemplo.test".to_owned());
        let _state = AppState::new(pool.clone(), missing_user).await?;

        db.cleanup().await?;
        Ok(())
    }
}
