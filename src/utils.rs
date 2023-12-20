use base64::prelude::*;
use regex::Regex;

use crate::models::ReindeerCompetitor;

// Day 4
pub fn get_fastest(contest: &[ReindeerCompetitor]) -> String {
    let res = contest
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .map(|winner| (winner.name.clone(), winner.strength))
        .unwrap_or_default();

    format!("Speeding past the finish line with a strength of {} is {}", res.1, res.0)
}

pub fn get_tallest(contest: &[ReindeerCompetitor]) -> String {
    let res = contest
        .iter()
        .max_by(|a, b| a.height.cmp(&b.height))
        .map(|winner| (winner.name.clone(), winner.antler_width))
        .unwrap_or_default();

    format!("{} is standing tall with his {} cm wide antlers", res.0, res.1)
}

pub fn get_magician(contest: &[ReindeerCompetitor]) -> String {
    let res = contest
        .iter()
        .max_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power))
        .map(|winner| (winner.name.clone(), winner.snow_magic_power))
        .unwrap_or_default();

    format!("{} could blast you away with a snow magic power of {}", res.0, res.1)
}

pub fn get_consumer(contest: &[ReindeerCompetitor]) -> String {
    let res = contest
        .iter()
        .max_by(|a, b| a.candies_eaten_yesterday.cmp(&b.candies_eaten_yesterday))
        .map(|winner| (winner.name.clone(), winner.favorite_food.clone()))
        .unwrap_or_default();

    format!("{} ate lots of candies, but also some {}", res.0, res.1)
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
        start_search = end;
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

// Day 7
pub fn decode_recipe(input: &str) -> String {
    let encoded_recipe = input.split_once('=').unwrap().1;

    let eng = BASE64_STANDARD;
    let decoded_cookie = eng.decode(encoded_recipe).unwrap();

    String::from_utf8(decoded_cookie).unwrap()
}


pub fn min_element(dividend: [i64; 5], divisor: [i64; 5]) -> Option<i64> {
    let results = [
        dividend[0].checked_div(divisor[0]),
        dividend[1].checked_div(divisor[1]),
        dividend[2].checked_div(divisor[2]),
        dividend[3].checked_div(divisor[3]),
        dividend[4].checked_div(divisor[4]),
    ];

    results.iter().filter_map(|&x| x).min()
}

pub fn find_reminder(dividend: [i64; 5], divisor: [i64; 5], quotient: i64) -> [i64; 5] {
    [
        dividend[0] - (divisor[0] * quotient),
        dividend[1] - (divisor[1] * quotient),
        dividend[2] - (divisor[2] * quotient),
        dividend[3] - (divisor[3] * quotient),
        dividend[4] - (divisor[4] * quotient),
    ]
}