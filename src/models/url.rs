use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model mapping to the `urls` table.
#[derive(Debug, Clone, FromRow)]
pub struct Url {
    pub id: Uuid,
    pub short_code: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub click_count: i64,
    pub last_clicked_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}
