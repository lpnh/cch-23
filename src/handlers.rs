use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::ParsedPath;
use crate::{ContestWinners, Reindeer, ReindeerCompetitor};

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
pub async fn cube_the_bits(ParsedPath(packets): ParsedPath) -> impl IntoResponse {
    let status = StatusCode::OK;

    let xor_result = packets.iter().fold(0, |acc, &x| acc ^ x);
    let cubed_result = xor_result.pow(3);
    let body = cubed_result.to_string();

    (status, body)
}

// Day 4
pub async fn reindeers(Json(reindeers): Json<Vec<Reindeer>>) -> impl IntoResponse {
    let status = StatusCode::OK;
    let body = reindeers
        .iter()
        .fold(0, |acc, r| acc + r.strength)
        .to_string();

    (status, body)
}

pub async fn reindeers_contest(
    Json(reindeer_competitors): Json<Vec<ReindeerCompetitor>>,
) -> impl IntoResponse {
    let status = StatusCode::OK;
    let body = ContestWinners::result(reindeer_competitors);

    (status, Json(body))
}
