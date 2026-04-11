pub mod health;
pub mod redirect;
pub mod shorten;
pub mod stats;
pub mod urls;

use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::AppState;

/// Builds the complete application router with all routes and middleware.
pub fn create_router(state: Arc<AppState>) -> Router {
    let api_v1 = Router::new()
        .route("/health", get(health::health_check))
        .route("/shorten", post(shorten::create_short_url));

    Router::new()
        .nest("/api/v1", api_v1)
        .route("/:code", get(redirect::redirect))
        .with_state(state)
}