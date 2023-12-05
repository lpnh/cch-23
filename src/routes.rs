use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::*;

pub fn app() -> Router {
    Router::new()
        // Day -1
        .route("/", get(home))
        .route("/-1/error", get(fake_error))
        // Day 1
        .route("/1/*packets", get(cube_the_bits))
        // Day 4
        .route("/4/strength", post(reindeers))
        .route("/4/contest", post(reindeers_contest))
}
