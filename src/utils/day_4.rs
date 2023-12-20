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
