use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let card_values = "123456789TJQKA";
    let hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let mut hand_type = cards
                .chars()
                .map(|card| cards.chars().filter(|&c| c == card).count())
                .collect::<Vec<_>>();
            hand_type.sort();
            hand_type = hand_type.into_iter().rev().collect::<Vec<_>>();
            let card_values = cards
                .chars()
                .map(|card| card_values.chars().position(|c| c == card).unwrap())
                .collect::<Vec<_>>();
            (hand_type, card_values, bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    calc_winnings(hands)
}

fn part2(input: &str) -> u32 {
    let card_values = "J123456789TQKA";
    let hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();

            let card_values = cards
                .chars()
                .map(|card| card_values.chars().position(|c| c == card).unwrap())
                .collect::<Vec<_>>();

            // Replace jokers with the most frequent card
            let most_frequent_card = cards
                .chars()
                .fold(HashMap::new(), |mut acc, ch| {
                    if ch != 'J' {
                        *acc.entry(ch).or_insert(0) += 1;
                    }
                    acc
                })
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(ch, _)| ch)
                .unwrap_or('J');
            let cards = cards.replace('J', &most_frequent_card.to_string());

            let mut hand_type = cards
                .chars()
                .map(|card| cards.chars().filter(|&c| c == card).count())
                .collect::<Vec<_>>();
            hand_type.sort();
            hand_type = hand_type.into_iter().rev().collect::<Vec<_>>();

            (hand_type, card_values, bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    calc_winnings(hands)
}

fn calc_winnings(mut hands: Vec<(Vec<usize>, Vec<usize>, u32)>) -> u32 {
    hands.sort_by(|(type_a, card_a, _), (type_b, card_b, _)| {
        for (a, b) in type_a.iter().zip(type_b) {
            if a != b {
                return a.cmp(b);
            }
        }
        for (a, b) in card_a.iter().zip(card_b) {
            if a != b {
                return a.cmp(b);
            }
        }
        std::cmp::Ordering::Equal
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i as u32 + 1) * bid)
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
        assert_eq!(part1(input), 6440);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 5905);
    }
}
