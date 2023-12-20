use serde::{Deserialize, Serialize};
use axum::Json;
use std::collections::HashMap;

use crate::utils::*;

#[derive(Serialize, Deserialize)]
pub struct Reindeer {
    name: String,
    pub strength: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ReindeerCompetitor {
    pub name: String,
    pub strength: i32,
    pub speed: f32,
    pub height: i32,
    pub antler_width: i32,
    pub snow_magic_power: i32,
    pub favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies_eaten_yesterday: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ContestWinners {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl ContestWinners {
    pub fn result(contest: Vec<ReindeerCompetitor>) -> Self {
        Self {
            fastest: day_4::get_fastest(&contest),
            tallest: day_4::get_tallest(&contest),
            magician: day_4::get_magician(&contest),
            consumer: day_4::get_consumer(&contest),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Shelf {
    elf: usize
}

impl Shelf {
    pub fn new(request: &str) -> Json<Shelf> {
        Json(
           Self { elf: day_6::count_elves(request) }
        )   
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShelvesAndElves {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelves_with_no_elf: usize,
}

impl ShelvesAndElves {
    pub fn new(request: &str) -> Json<ShelvesAndElves> {
        Json(
            Self {
                elf: day_6::count_elves(request),
                elf_on_a_shelf: day_6::count_elves_on_a_shelf(request),
                shelves_with_no_elf: day_6::count_shelves_with_no_elf(request)
            }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecipeAndPantry {
    pub recipe: HashMap<String, i64>,
    pub pantry: HashMap<String, i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookiesAndPantry {
    pub cookies: i64,
    pub pantry: HashMap<String, i64>,
}

impl CookiesAndPantry {
    pub fn from_recipe(recipe: &str) -> Json<Self> {
        let cookies_and_pantry = day_7::bake(recipe).unwrap();
        
        Json(
            Self {
                cookies: cookies_and_pantry.0,
                pantry: cookies_and_pantry.1,
            }
        )
    }
}
