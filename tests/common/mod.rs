use std::sync::Arc;
use std::time::Instant;

use axum::Router;
use sqlx::PgPool;
use url_shortener::AppState;
use url_shortener::config::AppConfig;
use url_shortener::routes::create_router;

/// Creates a test router with real DB and Redis connections.
pub async fn setup_test_app() -> (Router, Arc<AppState>) {
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();

    let db = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to test database");

    let redis = redis::Client::open(config.redis_url.as_str())
        .expect("Invalid Redis URL")
        .get_connection_manager()
        .await
        .expect("Failed to connect to test Redis");

    let state = Arc::new(AppState {
        db,
        redis,
        config,
        start_time: Instant::now(),
    });

    let router = create_router(state.clone());
    (router, state)
}

/// Cleans up database tables and Redis before each test.
pub async fn cleanup(state: &Arc<AppState>) {
    sqlx::query("TRUNCATE TABLE click_events, urls RESTART IDENTITY CASCADE")
        .execute(&state.db)
        .await
        .expect("Failed to truncate tables");

    let mut conn = state.redis.clone();
    let _: () = redis::cmd("FLUSHDB")
        .query_async(&mut conn)
        .await
        .expect("Failed to flush Redis");
}
