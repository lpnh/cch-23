use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::Path;

// Day -1
pub async fn home() -> &'static str {
    "ho ho ho!"
}

pub async fn fake_error() -> impl IntoResponse {
    let status = StatusCode::INTERNAL_SERVER_ERROR;
    let body = "ho ho... oh no!";

    (status, body)
}

// Day 1
pub async fn cube_the_bits(
    Path((num1, num2)): Path<(u32, u32)>,
) -> impl IntoResponse {
    let status = StatusCode::OK;

    let result = (num1 ^ num2).pow(3);
    let body = result.to_string();

    (status, body)
}
