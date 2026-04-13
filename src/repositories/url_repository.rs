use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Url;

/// Creates a new shortened URL record in the database.
pub async fn create_url(
    pool: &PgPool,
    short_code: &str,
    original_url: &str,
    expires_at: Option<DateTime<Utc>>,
) -> Result<Url, sqlx::Error> {
    sqlx::query_as::<_, Url>(
        "INSERT INTO urls (short_code, original_url, expires_at) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(short_code)
    .bind(original_url)
    .bind(expires_at)
    .fetch_one(pool)
    .await
}

/// Finds an active, non-expired URL by its original URL.
/// Used for duplicate detection — prevents shortening the same URL twice.
pub async fn find_by_original_url(
    pool: &PgPool,
    original_url: &str,
) -> Result<Option<Url>, sqlx::Error> {
    sqlx::query_as::<_, Url>(
        "SELECT * FROM urls WHERE original_url = $1 AND is_active = TRUE AND (expires_at IS NULL OR expires_at > NOW())",
    )
    .bind(original_url)
    .fetch_optional(pool)
    .await
}

/// Finds an active URL by its short code.
pub async fn find_by_short_code(
    pool: &PgPool,
    short_code: &str,
) -> Result<Option<Url>, sqlx::Error> {
    sqlx::query_as::<_, Url>("SELECT * FROM urls WHERE short_code = $1 AND is_active = TRUE")
        .bind(short_code)
        .fetch_optional(pool)
        .await
}

/// Checks if a short code already exists (including inactive ones).
pub async fn short_code_exists(pool: &PgPool, short_code: &str) -> Result<bool, sqlx::Error> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM urls WHERE short_code = $1")
        .bind(short_code)
        .fetch_one(pool)
        .await?;

    Ok(count > 0)
}

/// Increments the click count and updates last_clicked_at timestamp.
pub async fn increment_click_count(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE urls SET click_count = click_count + 1, last_clicked_at = NOW() WHERE id = $1",
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Soft deletes a URL by setting is_active = false.
pub async fn soft_delete(pool: &PgPool, short_code: &str) -> Result<bool, sqlx::Error> {
    let result =
        sqlx::query("UPDATE urls SET is_active = FALSE WHERE short_code = $1 AND is_active = TRUE")
            .bind(short_code)
            .execute(pool)
            .await?;

    Ok(result.rows_affected() > 0)
}

/// Lists URLs with pagination, ordered by created_at descending.
pub async fn list_urls(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Url>, sqlx::Error> {
    sqlx::query_as::<_, Url>(
        "SELECT * FROM urls WHERE is_active = TRUE ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

/// Returns the total count of active URLs (for pagination metadata).
pub async fn count_active_urls(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM urls WHERE is_active = TRUE")
        .fetch_one(pool)
        .await
}

/// Soft deletes all expired URLs that are still active.
/// Returns the number of URLs that were deactivated.
pub async fn cleanup_expired_urls(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE urls SET is_active = FALSE WHERE is_active = TRUE AND expires_at IS NOT NULL AND expires_at < NOW()",
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

/// Increments click count by a given delta for a short code.
/// Used by the background flush task to batch-update counts from Redis.
pub async fn increment_click_count_by_code(
    pool: &PgPool,
    short_code: &str,
    delta: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE urls SET click_count = click_count + $1, last_clicked_at = NOW() WHERE short_code = $2 AND is_active = TRUE",
    )
    .bind(delta)
    .bind(short_code)
    .execute(pool)
    .await?;

    Ok(())
}