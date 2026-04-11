use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

use crate::AppState;
use crate::dto::response::UrlStatsResponse;
use crate::errors::AppError;
use crate::services::url_service;

/// GET /api/v1/stats/:code
/// Returns URL details and recent click events.
pub async fn get_url_stats(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<Json<UrlStatsResponse>, AppError> {
    let stats = url_service::get_url_stats(&state, &code).await?;
    Ok(Json(stats))
}
