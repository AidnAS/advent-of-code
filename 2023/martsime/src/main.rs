use std::env;

use aoc::solution::Solution;
use aoc::solver;

const DAYS: u8 = 3;

struct ParsedArguments {
    pub day: Option<u8>,
    pub part: Option<u8>,
}

impl ParsedArguments {
    pub fn new() -> Self {
        let mut args = env::args().skip(1);
        let day = args.next().map(|s| s.parse::<u8>().expect("day is number"));
        if let Some(d) = day {
            assert!(d <= DAYS)
        };
        let part = args
            .next()
            .map(|s| s.parse::<u8>().expect("part is number"));
        if let Some(p) = part {
            assert!(p <= 2)
        };
        Self { day, part }
    }

    pub fn problems(&self) -> Vec<(u8, u8)> {
        match (self.day, self.part) {
            (Some(d), Some(p)) => vec![(d, p)],
            (Some(d), None) => vec![(d, 1), (d, 2)],
            (_, _) => (1..=DAYS).flat_map(|d| [(d, 1), (d, 2)]).collect(),
        }
    }
}

fn get_function(day: u8, part: u8) -> fn() -> Solution {
    match (day, part) {
        (1, 1) => aoc::day01::part01,
        (1, 2) => aoc::day01::part02,
        (2, 1) => aoc::day02::part01,
        (2, 2) => aoc::day02::part02,
        (3, 1) => aoc::day03::part01,
        (3, 2) => aoc::day03::part02,
        (_, _) => unimplemented!(),
    }
}

fn main() {
    let args = ParsedArguments::new();
    let problems = args.problems();

    for (day, part) in problems {
        let function = get_function(day, part);
        let result = solver::solve(day, part, function);
        println!("{}", result)
    }
}
