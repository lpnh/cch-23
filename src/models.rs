use serde::{Deserialize, Serialize};
use axum::Json;

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
        let fastest = get_fastest(&contest);
        let tallest = get_tallest(&contest);
        let magician = get_magician(&contest);
        let consumer = get_consumer(&contest);

        Self {
            fastest: format!(
                "Speeding past the finish line with a strength of {} is {}",
                fastest.1, fastest.0
            ),
            tallest: format!(
                "{} is standing tall with his {} cm wide antlers",
                tallest.0, tallest.1
            ),
            magician: format!(
                "{} could blast you away with a snow magic power of {}",
                magician.0, magician.1
            ),
            consumer: format!("{} ate lots of candies, but also some {}",
                consumer.0, consumer.1
            ),
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
           Self { elf: count_elves(request) }
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
                elf: count_elves(request),
                elf_on_a_shelf: count_elves_on_a_shelf(request),
                shelves_with_no_elf: count_shelves_with_no_elf(request)
            }
        )
    }
}
