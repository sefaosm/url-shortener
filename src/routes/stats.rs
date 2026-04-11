use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

use crate::AppState;
use crate::dto::response::UrlStatsResponse;
use crate::errors::AppError;
use crate::services::url_service;

/// Get URL statistics and recent click events
#[utoipa::path(
    get,
    path = "/api/v1/stats/{code}",
    tag = "URLs",
    params(
        ("code" = String, Path, description = "The short code to get stats for"),
    ),
    responses(
        (status = 200, description = "URL statistics retrieved", body = UrlStatsResponse),
        (status = 404, description = "Short code not found"),
        (status = 429, description = "Rate limit exceeded"),
    )
)]
pub async fn get_url_stats(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<Json<UrlStatsResponse>, AppError> {
    let stats = url_service::get_url_stats(&state, &code).await?;
    Ok(Json(stats))
}
