use serde::Deserialize;
use utoipa::ToSchema;

/// Request body for POST /api/v1/shorten
#[derive(Debug, Deserialize, ToSchema)]
pub struct ShortenRequest {
    /// The original URL to shorten (must be http or https)
    pub url: String,
    /// Optional custom short code (alphanumeric, max 32 chars)
    pub custom_code: Option<String>,
    /// Optional expiration time in hours
    pub expires_in_hours: Option<i64>,
}

/// Query parameters for GET /api/v1/urls
#[derive(Deserialize, ToSchema)]
pub struct PaginationParams {
    /// Page number (default: 1)
    pub page: Option<u32>,
    /// Items per page (default: 20, max: 100)
    pub per_page: Option<u32>,
}