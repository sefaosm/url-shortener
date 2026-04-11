use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

use crate::dto::request::ShortenRequest;
use crate::dto::response::ShortenResponse;
use crate::errors::AppError;
use crate::services::url_service;
use crate::AppState;

/// POST /api/v1/shorten
/// Creates a new shortened URL.
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