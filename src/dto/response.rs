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