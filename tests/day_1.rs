#[cfg(test)]
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

use cch23_lpnh::app;

// Task 1
#[tokio::test]
async fn cube_the_bits_1() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/1/4/8")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"1728");
}

// Task 2
#[tokio::test]
async fn cube_the_bits_2() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/1/10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"1000");
}

#[tokio::test]
async fn cube_the_bits_3() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/1/4/5/8/10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"27");
}
