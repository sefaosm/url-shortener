use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::dto::request::PaginationParams;
use crate::dto::response::UrlListResponse;
use crate::errors::AppError;
use crate::services::url_service;
use crate::AppState;

/// GET /api/v1/urls
/// Lists all URLs with pagination.
pub async fn list_urls(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<UrlListResponse>, AppError> {
    let response = url_service::list_urls(&state, params.page, params.per_page).await?;
    Ok(Json(response))
}

/// DELETE /api/v1/urls/:code
/// Soft deletes a URL.
pub async fn delete_url(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<StatusCode, AppError> {
    url_service::delete_url(&state, &code).await?;
    Ok(StatusCode::NO_CONTENT)
}