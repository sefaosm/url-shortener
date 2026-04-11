use std::sync::Arc;

use anyhow::Context;
use redis::Client;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use url_shortener::{AppState, config::AppConfig, routes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load .env file (silently ignore if missing — production won't have it)
    dotenvy::dotenv().ok();

    // 2. Initialize structured logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 3. Load configuration
    let config = AppConfig::from_env();
    tracing::info!(
        "Configuration loaded, starting server at {}",
        config.server_addr()
    );

    // 4. Create PostgreSQL connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await
        .context("Failed to connect to PostgreSQL")?;
    tracing::info!("Connected to PostgreSQL");

    // 5. Run database migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .context("Failed to run database migrations")?;
    tracing::info!("Database migrations applied");

    // 6. Create Redis connection
    let redis_client = Client::open(config.redis_url.as_str()).context("Invalid Redis URL")?;
    let redis_conn = redis::aio::ConnectionManager::new(redis_client)
        .await
        .context("Failed to connect to Redis")?;
    tracing::info!("Connected to Redis");

    // 7. Build application state
    let state = Arc::new(AppState {
        db: db_pool,
        redis: redis_conn,
        config: config.clone(),
        start_time: std::time::Instant::now(),
    });

    // 8. Build router
    let router = routes::create_router(state.clone());

    // 9. Spawn background tasks
    tokio::spawn(url_shortener::tasks::cleanup::run_expired_url_cleanup(
        state.clone(),
    ));
    tokio::spawn(url_shortener::tasks::click_flush::run_click_count_flush(
        state.clone(),
    ));

    // 10. Start server with graceful shutdown
    let listener = TcpListener::bind(config.server_addr())
        .await
        .context("Failed to bind TCP listener")?;

    tracing::info!("🚀 Server listening on http://{}", config.server_addr());

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .context("Server error")?;

    tracing::info!("Server shut down gracefully");
    Ok(())
}

/// Listens for CTRL+C signal to trigger graceful shutdown.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    tracing::info!("Shutdown signal received, starting graceful shutdown...");
}
