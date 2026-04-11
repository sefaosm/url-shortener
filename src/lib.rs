pub mod config;
pub mod dto;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod tasks;

use config::AppConfig;

/// Shared application state passed to all handlers via Axum's State extractor.
#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: redis::aio::ConnectionManager,
    pub config: AppConfig,
    pub start_time: std::time::Instant,
}
