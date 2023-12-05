use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_winnings_from_line(line: &str) -> u32 {
    let (game_and_winning_numbers, numbers) = line.split('|').collect_tuple().expect("2 parts");

    let winning_numbers = game_and_winning_numbers
        .split(':')
        .last()
        .expect("Has tail element")
        .split_whitespace()
        .collect::<Vec<_>>();

    numbers
        .split_whitespace()
        .filter(|&num| winning_numbers.contains(&num))
        .count() as u32
}

pub fn solve() -> SolutionPair {
    // Part 1
    let reader = BufReader::new(File::open("input/input.txt").expect("File can be opened"));
    let points: u32 = reader
        .lines()
        .map(|line| get_winnings_from_line(&line.expect("Line can be read")))
        .filter(|&winnings| winnings > 0)
        .map(|winnings| 2u32.pow(winnings - 1))
        .sum();

    // Part 2
    let mut instances_per_card: Vec<u32> = Vec::new();
    let reader = BufReader::new(File::open("input/input.txt").expect("File can be opened"));

    for (card_index, line) in reader.lines().enumerate() {
        if card_index >= instances_per_card.len() {
            instances_per_card.push(1);
        } else {
            instances_per_card[card_index] += 1;
        }

        let num_instances = instances_per_card[card_index];
        let line = line.expect("Line can be read");
        let num_descending_copies = get_winnings_from_line(&line);

        for i in 1..=num_descending_copies {
            let target_index = card_index + i as usize;

            if target_index < instances_per_card.len() {
                instances_per_card[target_index] += num_instances;
            } else {
                instances_per_card.push(num_instances);
            }
        }
    }
    let num_cards = instances_per_card.iter().sum();

    (Solution::U32(points), Solution::U32(num_cards))
}
