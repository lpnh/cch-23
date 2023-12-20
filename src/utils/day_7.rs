use std::collections::HashMap;

use base64::prelude::*;
use serde_json::from_str;

use crate::models::RecipeAndPantry;

pub fn decode_recipe(input: &str) -> String {
    let encoded_recipe = input.split_once('=').unwrap().1;

    let eng = BASE64_STANDARD;
    let decoded_cookie = eng.decode(encoded_recipe).unwrap();

    String::from_utf8(decoded_cookie).unwrap()
}

pub fn bake(recipe_json: &str) -> Result<(i64, HashMap<String, i64>), &'static str> {
    let recipe_and_pantry: RecipeAndPantry = from_str(recipe_json)
        .map_err(|_| "Failed to parse JSON")?;

    let RecipeAndPantry { recipe, pantry } = recipe_and_pantry;

    let cookies = min_possible_cookies(&recipe, &pantry)
        .ok_or("Mismatch in recipe and pantry items")?;

    let pantry_remainder = pantry_after_baking(&recipe, &pantry, cookies)
        .ok_or("Error calculating pantry remainder")?;

    Ok((cookies, pantry_remainder))
}

fn min_possible_cookies(recipe: &HashMap<String, i64>, pantry: &HashMap<String, i64>) -> Option<i64> {
    recipe.iter().filter_map(|(ingredient, &amount_needed)| {
        pantry.get(ingredient).and_then(|&amount_available| {
            amount_available.checked_div(amount_needed)
        })
    }).min()
}

fn pantry_after_baking(recipe: &HashMap<String, i64>, pantry: &HashMap<String, i64>, cookies: i64) -> Option<HashMap<String, i64>> {
    let mut new_pantry = pantry.clone();
    for (ingredient, &amount_needed) in recipe {
        new_pantry.entry(ingredient.clone())
            .and_modify(|e| *e -= amount_needed * cookies);
    }
    Some(new_pantry)
}
