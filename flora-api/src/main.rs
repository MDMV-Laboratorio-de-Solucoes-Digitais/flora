//! Flora API - Entry point

use flora_api::routes;
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

    // Build router (state passed per-route via `.with()` middleware in full implementation)
    let app = routes::create_router();

    // Start server
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.app.host, config.app.port)).await?;
    tracing::info!(
        "Server running on http://{}:{}",
        config.app.host,
        config.app.port
    );

    axum::serve(listener, app).await?;

    Ok(())
}
