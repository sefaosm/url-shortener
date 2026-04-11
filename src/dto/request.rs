use serde::Deserialize;

/// Request body for POST /api/v1/shorten
#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
    pub custom_code: Option<String>,
    pub expires_in_hours: Option<i64>,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}