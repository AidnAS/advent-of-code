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
            let jokers = cards.chars().filter(|&c| c == 'J').count();
            let mut hand_type = cards
                .chars()
                .map(|card| cards.chars().filter(|&c| c == card && card != 'J').count())
                .collect::<Vec<_>>();
            hand_type.sort();
            hand_type = hand_type.into_iter().skip(jokers).rev().collect::<Vec<_>>();
            if jokers > 0 && jokers < 5 {
                let first_type = hand_type[0];
                let mut new_hand_type = vec![jokers + first_type; first_type + jokers];
                new_hand_type.extend_from_slice(&hand_type[first_type..]);
                hand_type = new_hand_type;
            } else if jokers == 5 {
                hand_type = vec![5; 5];
            }
            let card_values = cards
                .chars()
                .map(|card| card_values.chars().position(|c| c == card).unwrap())
                .collect::<Vec<_>>();
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
