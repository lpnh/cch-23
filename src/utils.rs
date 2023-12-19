use crate::models::ReindeerCompetitor;

use regex::Regex;

// Day 4
pub fn get_fastest(contest: &[ReindeerCompetitor]) -> (String, i32) {
    contest
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .map(|winner| (winner.name.clone(), winner.strength))
        .unwrap_or_default()
}

pub fn get_tallest(contest: &[ReindeerCompetitor]) -> (String, i32) {
    contest
        .iter()
        .max_by(|a, b| a.height.cmp(&b.height))
        .map(|winner| (winner.name.clone(), winner.antler_width))
        .unwrap_or_default()
}

pub fn get_magician(contest: &[ReindeerCompetitor]) -> (String, i32) {
    contest
        .iter()
        .max_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power))
        .map(|winner| (winner.name.clone(), winner.snow_magic_power))
        .unwrap_or_default()
}

pub fn get_consumer(contest: &[ReindeerCompetitor]) -> (String, String) {
    contest
        .iter()
        .max_by(|a, b| a.candies_eaten_yesterday.cmp(&b.candies_eaten_yesterday))
        .map(|winner| (winner.name.clone(), winner.favorite_food.clone()))
        .unwrap_or_default()
}

// Day 6
pub fn count_elves(input: &str) -> usize {
    let regex = Regex::new(r"elf").unwrap();
    regex.find_iter(input).count()
}

pub fn count_elves_on_a_shelf(input: &str) -> usize {
    let regex = Regex::new(r"elf on a ").unwrap();
    let mut count = 0;
    let mut start_search = 0;

    while let Some(mat) = regex.find_at(input, start_search) {
        let end = mat.end();
        if input[end..].starts_with("shelf") {
            count += 1;
        }
        start_search = end; // Continue searching from the end of "elf on a "
    }

    count
}

pub fn count_shelves_with_no_elf(input: &str) -> usize {
    let shelf_regex = Regex::new(r"shelf").unwrap();
    let elf_on_a_shelf = "elf on a ";

    shelf_regex.find_iter(input)
        .filter(|mat| {
            let start = mat.start();

            if start < elf_on_a_shelf.len() {
                return true;
            }

            &input[start - elf_on_a_shelf.len()..start] != elf_on_a_shelf
        })
        .count()
}
