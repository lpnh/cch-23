use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn home() -> &'static str {
    "ho ho ho!"
}

pub async fn fake_error() -> impl IntoResponse {
    let status = StatusCode::INTERNAL_SERVER_ERROR;
    let body = "ho ho... oh no!";

    (status, body)
}
