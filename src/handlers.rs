use axum::http::{header, StatusCode, HeaderMap};
use axum::response::IntoResponse;
use axum::Json;
use base64::prelude::*;
use serde_json::{json, to_string_pretty};

use crate::ParsedPath;
use crate::models::*;

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
    let pretty_body = to_string_pretty(&body).unwrap();

    (status, pretty_body)
}

// Day 6
pub async fn get_elves_and_shelves(input: String) -> impl IntoResponse {
    let response = ShelvesAndElves::new(&input);

    let status = StatusCode::OK;
    let body = response;

    (status, body)
}

// Day 7
pub async fn recipe(header: HeaderMap) -> impl IntoResponse {
    if let Some(cookie) = header.get(header::COOKIE) {
        let cookie_str = cookie.to_str().unwrap();

        let encoded_recipe = cookie_str.split_once('=').unwrap().1;

        let eng = BASE64_STANDARD;
        let decoded_cookie = eng.decode(encoded_recipe).unwrap();

        let response = String::from_utf8(decoded_cookie).unwrap();
        let status = StatusCode::OK;

        return (status, response)
    }

    let status = StatusCode::INTERNAL_SERVER_ERROR;

    (status, "No cookie found".into())
}
