pub mod health;
pub mod redirect;
pub mod shorten;
pub mod stats;
pub mod urls;

use std::sync::Arc;

use axum::{routing::{get, delete, post}, Router};

use crate::AppState;

/// Builds the complete application router with all routes and middleware.
pub fn create_router(state: Arc<AppState>) -> Router {
    let api_v1 = Router::new()
        .route("/health", get(health::health_check))
        .route("/shorten", post(shorten::create_short_url));

    Router::new()
        .nest("/api/v1", api_v1)
        .route("/api/v1/stats/:code", get(stats::get_url_stats))
        .route("/api/v1/urls", get(urls::list_urls))
        .route("/api/v1/urls/:code", delete(urls::delete_url))
        .route("/:code", get(redirect::redirect))
        .with_state(state)
}