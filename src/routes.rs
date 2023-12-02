use axum::{routing::get, Router};

use crate::handlers::*;

pub fn app() -> Router {
    Router::new()
        // Day -1
        .route("/", get(home))
        .route("/-1/error", get(fake_error))
        // Day 1
        .route("/1/:num1/:num2", get(cube_the_bits))
}
