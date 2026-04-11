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

/// List all URLs with pagination
#[utoipa::path(
    get,
    path = "/api/v1/urls",
    tag = "URLs",
    params(
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u32>, Query, description = "Items per page (default: 20, max: 100)"),
    ),
    responses(
        (status = 200, description = "URL list retrieved", body = UrlListResponse),
        (status = 429, description = "Rate limit exceeded"),
    )
)]
pub async fn list_urls(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<UrlListResponse>, AppError> {
    let response = url_service::list_urls(&state, params.page, params.per_page).await?;
    Ok(Json(response))
}

/// Soft delete a URL by short code
#[utoipa::path(
    delete,
    path = "/api/v1/urls/{code}",
    tag = "URLs",
    params(
        ("code" = String, Path, description = "The short code to delete"),
    ),
    responses(
        (status = 204, description = "URL deleted successfully"),
        (status = 404, description = "Short code not found"),
        (status = 429, description = "Rate limit exceeded"),
    )
)]
pub async fn delete_url(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<StatusCode, AppError> {
    url_service::delete_url(&state, &code).await?;
    Ok(StatusCode::NO_CONTENT)
}