use utoipa::OpenApi;

use crate::dto::request::{PaginationParams, ShortenRequest};
use crate::dto::response::{
    ClickDetail, HealthResponse, ShortenResponse, UrlListResponse, UrlStatsResponse, UrlSummary,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "URL Shortener API",
        version = "0.1.0",
        description = "A fast, production-ready URL shortener service built with Rust",
        license(name = "MIT"),
    ),
    paths(
        crate::routes::health::health_check,
        crate::routes::shorten::create_short_url,
        crate::routes::redirect::redirect,
        crate::routes::stats::get_url_stats,
        crate::routes::urls::list_urls,
        crate::routes::urls::delete_url,
    ),
    components(schemas(
        ShortenRequest,
        PaginationParams,
        ShortenResponse,
        HealthResponse,
        UrlStatsResponse,
        ClickDetail,
        UrlListResponse,
        UrlSummary,
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "URLs", description = "URL shortening and management"),
        (name = "Redirect", description = "Short URL redirection"),
    )
)]
pub struct ApiDoc;