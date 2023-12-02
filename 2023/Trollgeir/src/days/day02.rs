use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_possible(&self, max_values: &Round) -> bool {
        self.rounds
            .iter()
            .all(|round| round.is_possible(max_values))
    }

    fn min_set(&self) -> Round {
        let mut min_set = Round::default();
        for round in &self.rounds {
            min_set.blue = min_set.blue.max(round.blue);
            min_set.red = min_set.red.max(round.red);
            min_set.green = min_set.green.max(round.green);
        }
        min_set
    }
}

struct Round {
    blue: u32,
    green: u32,
    red: u32,
}
impl Round {
    fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }

    fn is_possible(&self, max_values: &Self) -> bool {
        self.blue <= max_values.blue && self.green <= max_values.green && self.red <= max_values.red
    }

    fn default() -> Self {
        Round {
            blue: 0,
            green: 0,
            red: 0,
        }
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id_str, rounds_str) = s.split(':').collect_tuple().expect("2 parts");

        let id = game_id_str
            .trim_start_matches("Game")
            .trim()
            .parse()
            .expect("ID can be parsed");

        let rounds = rounds_str
            .split(';')
            .map(|round_str| {
                let mut round = Round::default();
                round_str.trim().split(',').for_each(|color_count| {
                    let (count, color) = color_count
                        .split_whitespace()
                        .collect_tuple()
                        .expect("2 parts");
                    match color {
                        "blue" => round.blue = count.parse().expect("Count can be parsed"),
                        "green" => round.green = count.parse().expect("Count can be parsed"),
                        "red" => round.red = count.parse().expect("Count can be parsed"),
                        _ => panic!("Unknown color"),
                    }
                });
                round
            })
            .collect();

        Ok(Game { id, rounds })
    }
}

pub fn solve() -> SolutionPair {
    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));

    let max_values = Round {
        blue: 14,
        green: 13,
        red: 12,
    };

    let mut id_sum = 0;
    let mut power_sum = 0;

    for line in reader.lines() {
        let game = line
            .expect("Line can be read")
            .parse::<Game>()
            .expect("Game can be parsed");
        if game.is_possible(&max_values) {
            id_sum += game.id;
        }
        power_sum += game.min_set().power();
    }

    (Solution::U32(id_sum), Solution::U32(power_sum))
}
