advent_of_code::solution!(2);

use itertools::Itertools;

fn parse_numbers(string: &str) -> Vec<u32> {
    string
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn check_numbers_validity(numbers: &[u32]) -> bool {
    let illegal_diffs = numbers
        .iter()
        .tuple_windows()
        .any(|(a, b)| a.abs_diff(*b) > 3);

    let is_monotonic = numbers.iter().tuple_windows().all(|(a, b)| a < b)
        || numbers.iter().tuple_windows().all(|(a, b)| a > b);

    !illegal_diffs && is_monotonic
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let numbers = parse_numbers(line);
            check_numbers_validity(numbers.as_slice()) as u32
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let numbers = parse_numbers(line);
            (0..numbers.len()).any(|i| {
                let mut numbers_with_one_out = numbers.clone();
                numbers_with_one_out.remove(i);

                check_numbers_validity(numbers_with_one_out.as_slice())
            }) as u32
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
