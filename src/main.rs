use axum::{routing::get, Router};
use axum::response::IntoResponse;
use axum::http::StatusCode;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(home))
        .route("/-1/error", get(fake_error));

    Ok(router.into())
}

async fn home() -> &'static str {
    "ho ho ho!"
}

async fn fake_error() -> impl IntoResponse {
    let status = StatusCode::INTERNAL_SERVER_ERROR;
    let body = "ho ho... oh no!";

    (status, body)
}
