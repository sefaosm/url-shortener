use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

/// Response body for POST /api/v1/shorten
#[derive(Debug, Serialize, ToSchema)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Response body for GET /api/v1/health
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub database: String,
    pub cache: String,
}

/// Response body for GET /api/v1/stats/:code
#[derive(Serialize, ToSchema)]
pub struct UrlStatsResponse {
    pub short_code: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_clicked_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub click_count: i64,
    pub recent_clicks: Vec<ClickDetail>,
}

/// Individual click event detail within stats response
#[derive(Serialize, ToSchema)]
pub struct ClickDetail {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub clicked_at: DateTime<Utc>,
}

/// Response body for GET /api/v1/urls
#[derive(Serialize, ToSchema)]
pub struct UrlListResponse {
    pub urls: Vec<UrlSummary>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

/// Individual URL summary within list response
#[derive(Serialize, ToSchema)]
pub struct UrlSummary {
    pub short_code: String,
    pub original_url: String,
    pub click_count: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}
