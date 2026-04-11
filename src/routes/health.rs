use axum::{Json, extract::State};
use std::sync::Arc;

use crate::AppState;
use crate::dto::response::HealthResponse;
use crate::errors::AppError;

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
    )
)]
pub async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HealthResponse>, AppError> {
    // Check database connectivity
    let db_status = match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => "connected".to_string(),
        Err(_) => "disconnected".to_string(),
    };

    // Check Redis connectivity
    let cache_status = {
        let mut conn = state.redis.clone();
        match redis::cmd("PING").query_async::<String>(&mut conn).await {
            Ok(_) => "connected".to_string(),
            Err(_) => "disconnected".to_string(),
        }
    };

    let uptime = state.start_time.elapsed().as_secs();

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
        database: db_status,
        cache: cache_status,
    }))
}
