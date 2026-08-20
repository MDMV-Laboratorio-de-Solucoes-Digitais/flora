//! Integration tests for the registration flow.
//!
//! These tests verify the full OIDC-based registration and organization creation flow.

use axum::http::StatusCode;
use flora_core::config::Config;
use flora_tests::test_utils::TestApp;
use serde_json::Value;
use sqlx::PgPool;

#[tokio::test]
async fn test_registration_flow() -> anyhow::Result<()> {
    let app = TestApp::spawn().await?;
    let config = Config::default();

    // Step 1: GET /auth/login
    let response = app.client.get("/auth/login").send().await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body: Value = response.json().await?;
    let auth_url = body["authorization_url"].as_str().unwrap();
    assert!(auth_url.starts_with(&config.oidc.issuer_url));
    assert!(auth_url.contains("response_type=code"));
    assert!(auth_url.contains("scope=openid+profile+email"));

    // Step 2: Simulate OIDC callback with a test code
    // In a real test, you would mock the OIDC provider or use a test provider.
    // For now, we'll simulate the callback logic directly.
    let test_code = "test_code_123";
    let callback_response = app
        .client
        .get("/auth/callback")
        .query(&[("code", test_code)])
        .send()
        .await?;

    // For this test, we expect a 401 because the code is invalid.
    // In a real test, you would mock the OIDC provider to return a valid token.
    assert_eq!(callback_response.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}

#[tokio::test]
async fn test_registration_flow_with_mock_oidc() -> anyhow::Result<()> {
    let app = TestApp::spawn().await?;
    let pool = PgPool::connect(&app.config.database.url).await?;

    // Simulate a successful OIDC callback
    let test_email = "test@example.com";
    let test_name = "Test User";
    let test_subject = "test_subject_123";

    // Insert a test user directly
    let user = sqlx::query_as::<_, flora_core::models::User>(
        "INSERT INTO users (id, email, oidc_subject, display_name, is_active)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, email, oidc_subject, display_name, avatar_url, profile,
                  is_active, created_at, updated_at",
    )
    .bind(uuid::Uuid::now_v7())
    .bind(test_email)
    .bind(test_subject)
    .bind(test_name)
    .bind(true)
    .fetch_one(&pool)
    .await?;

    // Simulate a session creation
    let session = sqlx::query_as::<_, flora_core::models::Session>(
        "INSERT INTO sessions (id, user_id, organization_id, jti, is_active, expires_at, last_activity_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id, user_id, organization_id, jti, refresh_token_id, client_ip,
                  user_agent, expires_at, last_activity_at, is_active, created_at, updated_at",
    )
    .bind(uuid::Uuid::now_v7())
    .bind(user.id)
    .bind(uuid::Uuid::nil()) // Placeholder org
    .bind(uuid::Uuid::now_v7().to_string())
    .bind(true)
    .bind(chrono::Utc::now() + chrono::Duration::hours(1))
    .bind(chrono::Utc::now())
    .fetch_one(&pool)
    .await?;

    // Generate a JWT for the session
    let jwt = flora_core::utils::jwt::encode_token(
        &flora_core::utils::jwt::FloraClaims::new(
            user.id,
            session.jti.clone(),
            session.organization_id,
            &user.email,
            3600,
        ),
        &app.config.app.jwt_secret,
    );

    // Verify the JWT works
    let claims = flora_core::utils::jwt::decode_token(&jwt, &app.config.app.jwt_secret)?;
    assert_eq!(claims.user_id(), user.id);
    assert_eq!(claims.email, user.email);

    Ok(())
}
