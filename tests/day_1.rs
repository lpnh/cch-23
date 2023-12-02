#[cfg(test)]
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

use cch23_lpnh::app;

#[tokio::test]
async fn cube_the_bits() {
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
