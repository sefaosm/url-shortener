use chrono::{Duration, Utc};
use std::sync::Arc;

use crate::errors::AppError;
use crate::models::Url;
use crate::repositories::url_repository;
use crate::services::{cache_service, code_generator};
use crate::AppState;

/// Maximum number of retries when a generated code collides.
const MAX_COLLISION_RETRIES: u32 = 5;

/// Creates a new shortened URL.
/// If `custom_code` is provided, validates and uses it.
/// Otherwise generates a random short code with collision handling.
pub async fn create_short_url(
    state: &Arc<AppState>,
    original_url: &str,
    custom_code: Option<&str>,
    expires_in_hours: Option<i64>,
) -> Result<Url, AppError> {
    // Validate the original URL
    validate_url(original_url)?;

    // Calculate expiration time
    let expires_at = expires_in_hours.map(|hours| Utc::now() + Duration::hours(hours));

    // Determine the short code
    let short_code = match custom_code {
        Some(code) => {
            code_generator::validate_custom_code(code, state.config.max_custom_code_length)
                .map_err(AppError::ValidationError)?;

            if url_repository::short_code_exists(&state.db, code).await? {
                return Err(AppError::AlreadyExists(code.to_string()));
            }

            code.to_string()
        }
        None => generate_unique_code(state).await?,
    };

    // Insert into database
    let url = url_repository::create_url(&state.db, &short_code, original_url, expires_at).await?;

    Ok(url)
}

/// Resolves a short code to the original URL.
/// Uses Redis cache first, falls back to PostgreSQL.
/// Records the click event asynchronously.
pub async fn resolve_url(
    state: &Arc<AppState>,
    short_code: &str,
    ip_address: Option<String>,
    user_agent: Option<String>,
    referer: Option<String>,
) -> Result<String, AppError> {
    // 1. Check Redis cache
    let mut redis = state.redis.clone();
    if let Some(cached_url) = cache_service::get_cached_url(&mut redis, short_code).await {
        // Fire-and-forget: record click and increment counter
        let state_clone = state.clone();
        let code = short_code.to_string();
        tokio::spawn(async move {
            record_click_async(&state_clone, &code, ip_address, user_agent, referer).await;
        });

        return Ok(cached_url);
    }

    // 2. Cache miss — query PostgreSQL
    let url = url_repository::find_by_short_code(&state.db, short_code)
        .await?
        .ok_or(AppError::NotFound)?;

    // 3. Check expiration
    if let Some(expires_at) = url.expires_at {
        if expires_at < Utc::now() {
            return Err(AppError::ExpiredUrl);
        }
    }

    // 4. Cache the result in Redis
    cache_service::set_cached_url(&mut redis, short_code, &url.original_url).await;

    // 5. Fire-and-forget: record click
    let state_clone = state.clone();
    let code = short_code.to_string();
    let url_id = url.id;
    let original_url = url.original_url.clone();
    tokio::spawn(async move {
        record_click_with_id(&state_clone, url_id, ip_address, user_agent, referer).await;
    });

    Ok(original_url)
}

/// Validates that the input is a proper HTTP/HTTPS URL.
fn validate_url(input: &str) -> Result<(), AppError> {
    let parsed = url::Url::parse(input).map_err(|_| {
        AppError::ValidationError("Invalid URL format".to_string())
    })?;

    match parsed.scheme() {
        "http" | "https" => Ok(()),
        _ => Err(AppError::ValidationError(
            "URL must use http or https scheme".to_string(),
        )),
    }
}

/// Generates a unique short code with collision retry logic.
/// Starts with default length, bumps to +1 after MAX_COLLISION_RETRIES failures.
async fn generate_unique_code(state: &Arc<AppState>) -> Result<String, AppError> {
    let mut length = state.config.default_code_length;

    for attempt in 1..=(MAX_COLLISION_RETRIES + 1) {
        let code = code_generator::generate_short_code(length);

        if !url_repository::short_code_exists(&state.db, &code).await? {
            return Ok(code);
        }

        tracing::warn!(
            "Short code collision on attempt {} (length={}): '{}'",
            attempt,
            length,
            code
        );

        // After max retries at current length, bump length by 1
        if attempt == MAX_COLLISION_RETRIES {
            length += 1;
            tracing::warn!("Increasing code length to {}", length);
        }
    }

    Err(AppError::InternalError(anyhow::anyhow!(
        "Failed to generate unique short code after retries"
    )))
}

/// Records a click event when we got the URL from cache (need to look up url_id).
async fn record_click_async(
    state: &Arc<AppState>,
    short_code: &str,
    ip_address: Option<String>,
    user_agent: Option<String>,
    referer: Option<String>,
) {
    // We need the url_id to record the click event
    let url = match url_repository::find_by_short_code(&state.db, short_code).await {
        Ok(Some(url)) => url,
        _ => return,
    };

    record_click_with_id(state, url.id, ip_address, user_agent, referer).await;
}

/// Records a click event and increments the counter.
async fn record_click_with_id(
    state: &Arc<AppState>,
    url_id: uuid::Uuid,
    ip_address: Option<String>,
    user_agent: Option<String>,
    referer: Option<String>,
) {
    // Increment click count in DB
    if let Err(e) = url_repository::increment_click_count(&state.db, url_id).await {
        tracing::error!("Failed to increment click count: {}", e);
    }

    // Record detailed click event
    if let Err(e) = crate::repositories::click_repository::record_click(
        &state.db,
        url_id,
        ip_address.as_deref(),
        user_agent.as_deref(),
        referer.as_deref(),
    )
    .await
    {
        tracing::error!("Failed to record click event: {}", e);
    }
}