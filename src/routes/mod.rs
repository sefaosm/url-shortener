pub mod health;
pub mod redirect;
pub mod shorten;
pub mod stats;
pub mod urls;

use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;
use crate::openapi::ApiDoc;

/// Builds the complete application router with all routes and middleware.
pub fn create_router(state: Arc<AppState>) -> Router {
    // Rate limiter for URL creation: 10 requests per minute per IP
    let shorten_limiter = GovernorConfigBuilder::default()
        .per_second(6)
        .burst_size(10)
        .finish()
        .expect("Failed to build shorten rate limiter");

    // Rate limiter for general API routes: 30 requests per minute per IP
    let api_limiter = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(30)
        .finish()
        .expect("Failed to build API rate limiter");

    // Rate limiter for redirects: 60 requests per minute per IP
    let redirect_limiter = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(60)
        .finish()
        .expect("Failed to build redirect rate limiter");

    let shorten_route = Router::new()
        .route("/shorten", post(shorten::create_short_url))
        .layer(GovernorLayer {
            config: Arc::new(shorten_limiter),
        });

    let api_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/stats/:code", get(stats::get_url_stats))
        .route("/urls", get(urls::list_urls))
        .route("/urls/:code", delete(urls::delete_url))
        .layer(GovernorLayer {
            config: Arc::new(api_limiter),
        });

    let redirect_route = Router::new()
        .route("/:code", get(redirect::redirect))
        .layer(GovernorLayer {
            config: Arc::new(redirect_limiter),
        });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::DELETE,
        ])
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", shorten_route)
        .nest("/api/v1", api_routes)
        .merge(redirect_route)
        .with_state(state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
