use std::collections::HashSet;

fn matching_cards(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let (winning_numbers, numbers) =
                line.split(": ").nth(1).unwrap().split_once(" | ").unwrap();
            let winning_numbers = winning_numbers
                .split(' ')
                .filter_map(|number| number.parse::<u32>().ok())
                .collect::<HashSet<_>>();
            numbers
                .split(' ')
                .filter_map(|number| {
                    if let Ok(parsed) = number.parse::<u32>() {
                        if winning_numbers.contains(&parsed) {
                            return Some(parsed);
                        }
                    }
                    None
                })
                .count()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> u32 {
    matching_cards(input)
        .iter()
        .map(|number| 1 << number >> 1)
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let cards = matching_cards(input);
    cards
        .iter()
        .enumerate()
        .fold(vec![1; cards.len()], |mut acc, (i, card)| {
            (0..*card).for_each(|n| acc[i + n + 1] += acc[i]);
            acc
        })
        .iter()
        .sum::<u32>()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 30);
    }
}
