use axum::http::{header, StatusCode, HeaderMap, Response};
use axum::response::IntoResponse;
use axum::Json;
use axum::extract::Path;
use serde_json::to_string_pretty;
use rustemon::model::pokemon::Pokemon;
use tracing::{info, error};

use crate::ParsedPath;
use crate::models::*;
use crate::utils::day_7;

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
    let xor_result = packets.iter().fold(0, |acc, &x| acc ^ x);
    let cubed_result = xor_result.pow(3);
    let body = cubed_result.to_string();

    (StatusCode::OK, body)
}

// Day 4
pub async fn reindeers(Json(reindeers): Json<Vec<Reindeer>>) -> impl IntoResponse {
    let body = reindeers
        .iter()
        .fold(0, |acc, r| acc + r.strength)
        .to_string();

    (StatusCode::OK, body)
}

pub async fn reindeers_contest(
    Json(reindeer_competitors): Json<Vec<ReindeerCompetitor>>,
) -> impl IntoResponse {
    let body = ContestWinners::result(reindeer_competitors);
    let pretty_body = to_string_pretty(&body).unwrap();

    (StatusCode::OK, pretty_body)
}

// Day 6
pub async fn get_elves_and_shelves(input: String) -> impl IntoResponse {
    let response = ShelvesAndElves::new(&input);

    (StatusCode::OK, response)
}

// Day 7
pub async fn recipe(header: HeaderMap) -> impl IntoResponse {
    if let Some(cookie) = header.get(header::COOKIE) {
        let cookie_str = cookie.to_str().unwrap();
        let response = day_7::decode_recipe(cookie_str);

        return (StatusCode::OK, response)
    }

    let status = StatusCode::INTERNAL_SERVER_ERROR;

    (status, "No cookie found".into())
}

pub async fn baked_cookies(header: HeaderMap) -> impl IntoResponse {
    let cookie = header.get(header::COOKIE).unwrap();
    let cookie_str = cookie.to_str().unwrap();

    let recipe_json = day_7::decode_recipe(cookie_str);

    let response = CookiesAndPantry::from_recipe(&recipe_json);

    (StatusCode::OK, response)
}

// Day 8
pub async fn pokemon_weight(Path(id): Path<i64>) -> impl IntoResponse {
    info!(id);

    let poke_api_url = format!("https://pokeapi.co/api/v2/pokemon/{}/", id);

    match reqwest::get(&poke_api_url).await {
        Ok(api_response) => match api_response.json::<Pokemon>().await {
            Ok(pokemon) => {
                let pokemon_weight_kg = (pokemon.weight as f64 / 10.0).to_string();
                info!("Pokemon weight: {}", pokemon_weight_kg);
                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/plain")
                    .body(pokemon_weight_kg)
                    .unwrap()
            },
            Err(e) => {
                error!("Error parsing data: {}", e);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error".into())
                    .unwrap()
            },
        },
        Err(e) => {
            error!("Error fetching data from PokéAPI: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Error".into())
                .unwrap()
        },
    }
}
