use sqlx::PgPool;
use uuid::Uuid;

use crate::models::ClickEvent;

/// Records a click event for analytics.
pub async fn record_click(
    pool: &PgPool,
    url_id: Uuid,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
    referer: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO click_events (url_id, ip_address, user_agent, referer) VALUES ($1, $2::INET, $3, $4)",
    )
    .bind(url_id)
    .bind(ip_address)
    .bind(user_agent)
    .bind(referer)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetches recent click events for a given URL (for stats endpoint).
pub async fn get_recent_clicks(
    pool: &PgPool,
    url_id: Uuid,
    limit: i64,
) -> Result<Vec<ClickEvent>, sqlx::Error> {
    sqlx::query_as::<_, ClickEvent>(
        "SELECT * FROM click_events WHERE url_id = $1 ORDER BY clicked_at DESC LIMIT $2",
    )
    .bind(url_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}
