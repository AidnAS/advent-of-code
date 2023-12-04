use std::collections::HashSet;

use crate::solution::Solution;

fn parse_numbers(input: &str) -> HashSet<u32> {
    input
        .split(' ')
        .filter_map(|n| match n {
            "" => None,
            _ => Some(n.parse::<u32>().unwrap()),
        })
        .collect()
}

fn solve_part01(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, card) = line.split_once(": ").unwrap();
            let (winning_string, numbers_string) = card.split_once(" | ").unwrap();
            let (winning_numbers, numbers) =
                (parse_numbers(winning_string), parse_numbers(numbers_string));
            let numbers_matching = winning_numbers.intersection(&numbers).count() as u32;
            match numbers_matching {
                0 => 0,
                _ => 2u32.pow(numbers_matching - 1),
            }
        })
        .sum::<u32>()
}

fn solve_part02(input: &str) -> u32 {
    let num_cards = input.lines().count();
    let mut copies = vec![1; num_cards];
    input
        .lines()
        .enumerate()
        .map(|(card_index, line)| {
            let (_, card) = line.split_once(": ").unwrap();
            let (winning_string, numbers_string) = card.split_once(" | ").unwrap();
            let (winning_numbers, numbers) =
                (parse_numbers(winning_string), parse_numbers(numbers_string));
            let numbers_matching = winning_numbers.intersection(&numbers).count();
            let card_copies = copies[card_index];

            (card_index + 1..=(card_index + numbers_matching)).for_each(|i| {
                copies[i] += card_copies;
            });
            card_copies as u32
        })
        .sum::<u32>()
}

pub fn part01() -> Solution {
    solve_part01(include_str!("input.txt")).into()
}

pub fn part02() -> Solution {
    solve_part02(include_str!("input.txt")).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part01() {
        let input = include_str!("input_test.txt");
        let solution = solve_part01(input);
        assert_eq!(solution, 13);
    }

    #[test]
    fn day04_part02() {
        let input = include_str!("input_test.txt");
        let solution = solve_part02(input);
        assert_eq!(solution, 30);
    }
}
