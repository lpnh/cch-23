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
        let consumer: (String, i32) = get_consumer(&contest);

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

fn get_fastest(contest: &[ReindeerCompetitor]) -> (String, i32) {
    let mut winner_name = "".to_string();
    let mut winner_strength = 0;

    contest
        .iter()
        .fold((String::default(), 0.0), |acc, competitor| {
            if competitor.speed > acc.1 {
                winner_name = competitor.name.clone();
                winner_strength = competitor.strength;
                (competitor.name.clone(), competitor.speed)
            } else {
                acc
            }
        });

    (winner_name, winner_strength)
}

fn get_tallest(contest: &[ReindeerCompetitor]) -> (String, i32) {
    let mut winner_name = "".to_string();
    let mut winner_antler = 0;

    contest
        .iter()
        .fold((String::default(), 0), |acc, competitor| {
            if competitor.height > acc.1 {
                winner_name = competitor.name.clone();
                winner_antler = competitor.antler_width;
                (competitor.name.clone(), competitor.height)
            } else {
                acc
            }
        });
    
    (winner_name, winner_antler)
}

fn get_magician(contest: &[ReindeerCompetitor]) -> (String, i32) {
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

fn get_consumer(contest: &[ReindeerCompetitor]) -> (String, i32) {
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
