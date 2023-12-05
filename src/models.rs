use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Reindeer {
    name: String,
    pub strength: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ReindeerCompetitor {
    pub name: String,
    strength: u8,
    speed: f32,
    height: u8,
    antler_width: u8,
    snow_magic_power: u16,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u8,
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
            consumer: format!("{} ate lots of candies, but also some grass", consumer.0),
        }
    }
}

fn get_fastest(contest: &[ReindeerCompetitor]) -> (String, f32) {
    contest
        .iter()
        .fold((String::default(), 0.0), |acc, competitor| {
            if competitor.speed > acc.1 {
                (competitor.name.clone(), competitor.speed)
            } else {
                acc
            }
        })
}

fn get_tallest(contest: &[ReindeerCompetitor]) -> (String, u8) {
    contest
        .iter()
        .fold((String::default(), 0), |acc, competitor| {
            if competitor.height > acc.1 {
                (competitor.name.clone(), competitor.height)
            } else {
                acc
            }
        })
}

fn get_magician(contest: &[ReindeerCompetitor]) -> (String, u16) {
    contest
        .iter()
        .fold((String::default(), 0), |acc, competitor| {
            if competitor.snow_magic_power > acc.1 {
                (competitor.name.clone(), competitor.snow_magic_power)
            } else {
                acc
            }
        })
}

fn get_consumer(contest: &[ReindeerCompetitor]) -> (String, u8) {
    contest
        .iter()
        .fold((String::default(), 0), |acc, competitor| {
            if competitor.candies_eaten_yesterday > acc.1 {
                (competitor.name.clone(), competitor.candies_eaten_yesterday)
            } else {
                acc
            }
        })
}
