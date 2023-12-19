use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Reindeer {
    name: String,
    pub strength: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ReindeerCompetitor {
    pub name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
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

fn get_fastest(contest: &[ReindeerCompetitor]) -> (String, i32) {
    contest
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .map(|winner| (winner.name.clone(), winner.strength))
        .unwrap_or_default()
}

fn get_tallest(contest: &[ReindeerCompetitor]) -> (String, i32) {
    contest
        .iter()
        .max_by(|a, b| a.height.cmp(&b.height))
        .map(|winner| (winner.name.clone(), winner.antler_width))
        .unwrap_or_default()
}

fn get_magician(contest: &[ReindeerCompetitor]) -> (String, i32) {
    contest
        .iter()
        .max_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power))
        .map(|winner| (winner.name.clone(), winner.snow_magic_power))
        .unwrap_or_default()
}

fn get_consumer(contest: &[ReindeerCompetitor]) -> (String, String) {
    contest
        .iter()
        .max_by(|a, b| a.candies_eaten_yesterday.cmp(&b.candies_eaten_yesterday))
        .map(|winner| (winner.name.clone(), winner.favorite_food.clone()))
        .unwrap_or_default()
}
