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
        Self {
            fastest: get_fastest(&contest),
            tallest: get_tallest(&contest),
            magician: get_magician(&contest),
            consumer: get_consumer(&contest),
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

#[derive(Serialize, Deserialize)]
pub struct Ingredients {
    flour: i64,
    sugar: i64,
    butter: i64,
    #[serde(rename = "baking powder")]
    baking_powder: i64,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: i64,
}

impl Ingredients {
    fn to_array(&self) -> [i64; 5] {
        [
            self.flour,
            self.sugar,
            self.butter,
            self.baking_powder,
            self.chocolate_chips
        ]
    }

    pub fn from_array(array: [i64; 5]) -> Self {
        Self {
            flour: array[0],
            sugar: array[1],
            butter: array[2],
            baking_powder: array[3],
            chocolate_chips: array[4],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecipeAndPantry {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Serialize, Deserialize)]
pub struct CookiesAndPantry {
    cookies: i64,
    pantry: Ingredients,
}

impl CookiesAndPantry {
    pub fn bake(recipe_and_pantry: RecipeAndPantry) -> Json<Self> {
        let dividend = recipe_and_pantry.pantry.to_array();
        let divisor = recipe_and_pantry.recipe.to_array();
        
        let cookies = min_element(dividend, divisor).unwrap();
        let pantry_array = find_reminder(dividend, divisor, cookies);

        let pantry = Ingredients::from_array(pantry_array);

        Json (
            Self {
                cookies,
                pantry
            }
        )
    }
}
