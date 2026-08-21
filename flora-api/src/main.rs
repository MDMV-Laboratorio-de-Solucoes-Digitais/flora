//! Flora API - Entry point.

#![allow(
    clippy::allow_attributes,
    clippy::multiple_crate_versions,
    reason = "Transitive dependency conflicts are unfixable in code."
)]

use flora_api::routes;
use flora_api::state::AppState;
use flora_core::database::create_pool;
use flora_core::migrations::run_migrations;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "flora_api=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = flora_core::config::Config::load()?;
    config.validate()?;

    // Connect to PostgreSQL and run migrations
    let pool = create_pool(&config).await?;
    run_migrations(&pool).await?;

    // Build application state with DB and Valkey connections
    let app_state = AppState::new(config.clone()).await?;

    // Build router and start server
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.app.host, config.app.port)).await?;

    tracing::info!(
        "Flora API listening on http://{}:{}",
        config.app.host,
        config.app.port
    );

    axum::serve(listener, routes::create_router(app_state)).await?;

    Ok(())
}
