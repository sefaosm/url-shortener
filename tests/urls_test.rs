mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn create_short_url(router: &axum::Router, url: &str) -> String {
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/shorten")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::json!({ "url": url }).to_string()))
        .unwrap();

    let response = router.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    json["short_code"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn list_urls_returns_200_with_pagination() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    create_short_url(&router, "https://www.example1.com").await;
    create_short_url(&router, "https://www.example2.com").await;
    create_short_url(&router, "https://www.example3.com").await;

    let request = Request::builder()
        .uri("/api/v1/urls?page=1&per_page=2")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["urls"].as_array().unwrap().len(), 2);
    assert_eq!(json["total"], 3);
}

#[tokio::test]
async fn delete_url_returns_204() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    let code = create_short_url(&router, "https://www.to-delete.com").await;

    let request = Request::builder()
        .method("DELETE")
        .uri(format!("/api/v1/urls/{code}"))
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn delete_nonexistent_url_returns_404() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    let request = Request::builder()
        .method("DELETE")
        .uri("/api/v1/urls/nonexistent")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
