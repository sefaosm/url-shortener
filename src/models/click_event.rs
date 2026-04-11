use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model mapping to the `click_events` table.
#[derive(Debug, Clone, FromRow)]
pub struct ClickEvent {
    pub id: Uuid,
    pub url_id: Uuid,
    pub clicked_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
}
