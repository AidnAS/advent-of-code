use itertools::Itertools;
use regex::Regex;

use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;

fn create_engine_window_iterator_from_reader(
    reader: BufReader<File>,
) -> impl Iterator<Item = (String, String, String)> {
    // Iterator creates synthetic first and last lines with "." * data length.
    once(None)
        .chain(reader.lines().map(|result| result.ok()))
        .chain(once(None))
        .tuple_windows()
        .map(|(top, center, bottom)| {
            let center = center.expect("Has content");
            let top = top.unwrap_or_else(|| ".".repeat(center.len()));
            let bottom = bottom.unwrap_or_else(|| ".".repeat(center.len()));
            (top, center, bottom)
        })
}

pub fn solve() -> SolutionPair {
    let number_pattern = Regex::new(r"\d+").unwrap();
    let non_digit_pattern = Regex::new(r"[^.\d]").unwrap();
    let gear_pattern = Regex::new(r"\*").unwrap();

    // Part 1
    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let window_iterator = create_engine_window_iterator_from_reader(reader);

    let mut part_numbers_sum = 0;
    for (top, center, bottom) in window_iterator {
        for m in number_pattern.find_iter(&center) {
            // Create a box with +1 position in each direction around the match
            let box_start = m.start().saturating_sub(1);
            let box_end = (m.end() + 1).min(center.len());
            let all_lines = [&top, &center, &bottom];
            let found_adjecent_symbol = all_lines
                .iter()
                .any(|line| non_digit_pattern.is_match(line.get(box_start..box_end).unwrap()));

            if found_adjecent_symbol {
                part_numbers_sum += m.as_str().parse::<u32>().unwrap()
            }
        }
    }

    // Part 2
    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let window_iterator = create_engine_window_iterator_from_reader(reader);

    let mut gear_ratio_sum = 0;
    for (top, center, bottom) in window_iterator {
        for gear_match in gear_pattern.find_iter(&center) {
            // Find all the numbers on each line, then filter those that do not overlap with the gear match
            let adjecent_numbers = [&top, &center, &bottom]
                .iter()
                .flat_map(|line| number_pattern.find_iter(line))
                .filter(|m| m.start() <= gear_match.end() && gear_match.start() <= m.end())
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            if adjecent_numbers.len() == 2 {
                gear_ratio_sum += adjecent_numbers.iter().product::<u32>();
            }
        }
    }
    (
        Solution::U32(part_numbers_sum),
        Solution::U32(gear_ratio_sum),
    )
}
