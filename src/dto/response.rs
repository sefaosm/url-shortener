use chrono::{DateTime, Utc};
use serde::Serialize;

/// Response body for POST /api/v1/shorten
#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Response body for GET /api/v1/health
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub database: String,
    pub cache: String,
}

#[derive(Serialize)]
pub struct UrlStatsResponse {
    pub short_code: String,
    pub original_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_clicked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
    pub click_count: i64,
    pub recent_clicks: Vec<ClickDetail>,
}

#[derive(Serialize)]
pub struct ClickDetail {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct UrlListResponse {
    pub urls: Vec<UrlSummary>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[derive(Serialize)]
pub struct UrlSummary {
    pub short_code: String,
    pub original_url: String,
    pub click_count: i64,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}