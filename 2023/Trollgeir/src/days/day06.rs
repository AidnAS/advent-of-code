use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_numbers_in_line(line: &str) -> Vec<u64> {
    line.split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn get_distance_from_button_timings(max_hold_time: u64) -> impl Iterator<Item = u64> {
    (0..=max_hold_time).map(move |ms| (ms) * (max_hold_time - ms))
}

pub fn solve() -> SolutionPair {
    // Part 1
    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let (time, distance) = reader
        .lines()
        .map(|line| get_numbers_in_line(&line.unwrap()))
        .collect_tuple()
        .unwrap();

    let product: u64 = time
        .iter()
        .zip(distance.iter())
        .map(|(max_time, best_distance)| {
            get_distance_from_button_timings(*max_time)
                .filter(|distance| distance > best_distance)
                .count() as u64
        })
        .product();

    // Part 2
    let time = time
        .iter()
        .map(|&num| num.to_string())
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance = distance
        .iter()
        .map(|&num| num.to_string())
        .join("")
        .parse::<u64>()
        .unwrap();

    let solution_2 = get_distance_from_button_timings(time)
        .filter(|d| d > &distance)
        .count();

    (
        Solution::U32(product as u32),
        Solution::U32(solution_2 as u32),
    )
}
