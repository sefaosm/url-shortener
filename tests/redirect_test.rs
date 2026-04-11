mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// Helper: creates a short URL and returns the short_code
async fn create_short_url(router: &axum::Router) -> String {
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/shorten")
        .header("Content-Type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "url": "https://www.rust-lang.org"
            })
            .to_string(),
        ))
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    json["short_code"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn redirect_valid_code_returns_302() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    let code = create_short_url(&router).await;

    let request = Request::builder()
        .uri(format!("/{code}"))
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::FOUND);

    let location = response
        .headers()
        .get("location")
        .expect("Missing Location header")
        .to_str()
        .unwrap();

    assert_eq!(location, "https://www.rust-lang.org");
}

#[tokio::test]
async fn redirect_nonexistent_code_returns_404() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    let request = Request::builder()
        .uri("/nonexistent")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn redirect_expired_url_returns_410() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    // Create a URL, then manually expire it in the database
    let code = create_short_url(&router).await;

    sqlx::query("UPDATE urls SET expires_at = NOW() - INTERVAL '1 hour' WHERE short_code = $1")
        .bind(&code)
        .execute(&state.db)
        .await
        .expect("Failed to expire URL");

    // Also clear Redis cache so it hits the DB
    let mut conn = state.redis.clone();
    let cache_key = format!("url:{code}");
    let _: () = redis::cmd("DEL")
        .arg(&cache_key)
        .query_async(&mut conn)
        .await
        .expect("Failed to clear Redis cache");

    let request = Request::builder()
        .uri(format!("/{code}"))
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::GONE);
}