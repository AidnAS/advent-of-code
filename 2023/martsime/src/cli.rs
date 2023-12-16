use std::env;

use crate::solution::Solution;

pub const DAYS: u8 = 15;

pub struct ParsedArguments {
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

impl Default for ParsedArguments {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_function(day: u8, part: u8) -> fn() -> Solution {
    match (day, part) {
        (1, 1) => crate::day01::part01,
        (1, 2) => crate::day01::part02,
        (2, 1) => crate::day02::part01,
        (2, 2) => crate::day02::part02,
        (3, 1) => crate::day03::part01,
        (3, 2) => crate::day03::part02,
        (4, 1) => crate::day04::part01,
        (4, 2) => crate::day04::part02,
        (5, 1) => crate::day05::part01,
        (5, 2) => crate::day05::part02,
        (6, 1) => crate::day06::part01,
        (6, 2) => crate::day06::part02,
        (7, 1) => crate::day07::part01,
        (7, 2) => crate::day07::part02,
        (8, 1) => crate::day08::part01,
        (8, 2) => crate::day08::part02,
        (9, 1) => crate::day09::part01,
        (9, 2) => crate::day09::part02,
        (10, 1) => crate::day10::part01,
        (10, 2) => crate::day10::part02,
        (11, 1) => crate::day11::part01,
        (11, 2) => crate::day11::part02,
        (12, 1) => crate::day12::part01,
        (12, 2) => crate::day12::part02,
        (13, 1) => crate::day13::part01,
        (13, 2) => crate::day13::part02,
        (14, 1) => crate::day14::part01,
        (14, 2) => crate::day14::part02,
        (15, 1) => crate::day15::part01,
        (15, 2) => crate::day15::part02,
        (_, _) => unimplemented!(),
    }
}
