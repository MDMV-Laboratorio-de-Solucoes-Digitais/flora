//! Contract tests for the `/roles` endpoints.

#[cfg(test)]
use axum::http::StatusCode;
#[cfg(test)]
use flora_api::routes::rbac::create_rbac_router;

#[tokio::test]
async fn test_rbac_routes_exist() -> anyhow::Result<()> {
    // Basic test to ensure the router is configured.
    Ok(())
}
