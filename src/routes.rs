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
        // Day 6
        .route("/6", post(get_elves_and_shelves))
        // Day 7
        .route("/7/decode", get(recipe))
        .route("/7/bake", get(bake_cookies))
}
