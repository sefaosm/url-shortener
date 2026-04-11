mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn shorten_valid_url_returns_201() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

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

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["short_code"].is_string());
    assert!(json["short_url"].is_string());
    assert!(
        json["original_url"]
            .as_str()
            .unwrap()
            .contains("rust-lang.org")
    );
}

#[tokio::test]
async fn shorten_duplicate_url_returns_409() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    let build_request = || {
        Request::builder()
            .method("POST")
            .uri("/api/v1/shorten")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "url": "https://www.duplicate-test.com"
                })
                .to_string(),
            ))
            .unwrap()
    };

    // First request — should succeed
    let resp1 = router.clone().oneshot(build_request()).await.unwrap();
    assert_eq!(resp1.status(), StatusCode::CREATED);

    // Second request — same URL, should conflict
    let resp2 = router.oneshot(build_request()).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn shorten_invalid_url_returns_422() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/shorten")
        .header("Content-Type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "url": "not-a-valid-url"
            })
            .to_string(),
        ))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn shorten_empty_body_returns_422() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/shorten")
        .header("Content-Type", "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
