// src/routes/redirect.rs

use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::errors::AppError;
use crate::services::url_service;
use crate::AppState;

/// GET /:code
/// Redirects to the original URL.
/// Uses 302 Found instead of 301 so browsers don't cache the redirect,
/// allowing us to track every click.
pub async fn redirect(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let ip_address = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string());

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let referer = headers
        .get("referer")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let original_url = url_service::resolve_url(
        &state,
        &code,
        ip_address,
        user_agent,
        referer,
    )
    .await?;

    let location = HeaderValue::from_str(&original_url)
        .map_err(|_| AppError::InternalError(anyhow::Error::msg("Invalid URL stored in database".to_string())))?;

    Ok((StatusCode::FOUND, [(header::LOCATION, location)]).into_response())
}