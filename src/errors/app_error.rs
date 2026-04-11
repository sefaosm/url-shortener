use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// Centralized error type for the entire application.
/// Every variant maps to a specific HTTP status code and JSON body.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("The requested short URL was not found")]
    NotFound,

    #[error("The custom code '{0}' is already in use")]
    AlreadyExists(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("The requested short URL has expired")]
    ExpiredUrl,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal server error")]
    InternalError(#[from] anyhow::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Cache error: {0}")]
    CacheError(String),
}

/// JSON error body returned to the client.
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    status: u16,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_key) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "not_found"),
            AppError::AlreadyExists(_) => (StatusCode::CONFLICT, "already_exists"),
            AppError::ValidationError(_) => (StatusCode::UNPROCESSABLE_ENTITY, "validation_error"),
            AppError::ExpiredUrl => (StatusCode::GONE, "expired_url"),
            AppError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "rate_limit_exceeded"),
            AppError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            AppError::CacheError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "cache_error"),
        };

        // Log internal errors — don't expose details to client
        match &self {
            AppError::InternalError(e) => tracing::error!("Internal error: {:?}", e),
            AppError::DatabaseError(e) => tracing::error!("Database error: {:?}", e),
            AppError::CacheError(e) => tracing::error!("Cache error: {}", e),
            _ => {}
        }

        let body = ErrorResponse {
            error: error_key.to_string(),
            message: match &self {
                // Don't leak internal details to client
                AppError::InternalError(_)
                | AppError::DatabaseError(_)
                | AppError::CacheError(_) => "An unexpected error occurred".to_string(),
                other => other.to_string(),
            },
            status: status.as_u16(),
        };

        (status, Json(body)).into_response()
    }
}
