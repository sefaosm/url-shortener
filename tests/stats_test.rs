mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

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
async fn stats_returns_200_for_existing_url() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    let code = create_short_url(&router).await;

    let request = Request::builder()
        .uri(format!("/api/v1/stats/{code}"))
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["short_code"], code);
    assert!(
        json["original_url"]
            .as_str()
            .unwrap()
            .contains("rust-lang.org")
    );
    assert!(json["click_count"].is_number());
}

#[tokio::test]
async fn stats_returns_404_for_nonexistent_code() {
    let (router, state) = common::setup_test_app().await;
    common::cleanup(&state).await;

    let request = Request::builder()
        .uri("/api/v1/stats/nonexistent")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
