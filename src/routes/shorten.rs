use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

use crate::dto::request::ShortenRequest;
use crate::dto::response::ShortenResponse;
use crate::errors::AppError;
use crate::services::url_service;
use crate::AppState;

/// Create a new shortened URL
#[utoipa::path(
    post,
    path = "/api/v1/shorten",
    tag = "URLs",
    request_body = ShortenRequest,
    responses(
        (status = 201, description = "URL shortened successfully", body = ShortenResponse),
        (status = 409, description = "Custom code already exists"),
        (status = 422, description = "Invalid URL format"),
        (status = 429, description = "Rate limit exceeded"),
    )
)]
pub async fn create_short_url(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ShortenRequest>,
) -> Result<(StatusCode, Json<ShortenResponse>), AppError> {
    let url = url_service::create_short_url(
        &state,
        &payload.url,
        payload.custom_code.as_deref(),
        payload.expires_in_hours,
    )
    .await?;

    let response = ShortenResponse {
        short_code: url.short_code.clone(),
        short_url: format!("{}/{}", state.config.base_url, url.short_code),
        original_url: url.original_url,
        expires_at: url.expires_at,
        created_at: url.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}