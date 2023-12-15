use std::cmp::Ordering;

use crate::solution::Solution;

type Cards = [u8; 5];

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum CardHandStrength {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CardHandStrength {
    fn evaluate_strength(cards: &Cards, joker: bool) -> Self {
        let mut counts = [0u8; 15];
        for card in cards.iter() {
            counts[*card as usize] += 1;
        }
        // If the joker is a special card, convert all jokers to the most frequent card
        if joker {
            let num_jokers = counts[1];
            counts[1] = 0;
            let most_frequent_index = counts
                .iter()
                .enumerate()
                .skip(2)
                .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
                .unwrap()
                .0;
            counts[most_frequent_index] += num_jokers;
        }
        let mut pairs = 0u8;
        let mut three = false;
        let mut four = false;
        let mut five = false;
        for count in counts.iter().skip(2) {
            match count {
                2 => pairs += 1,
                3 => three = true,
                4 => four = true,
                5 => five = true,
                _ => {}
            }
        }
        match (five, four, three, pairs) {
            (true, _, _, _) => Self::FiveOfAKind,
            (_, true, _, _) => Self::FourOfAKind,
            (_, _, true, 1) => Self::FullHouse,
            (_, _, true, 0) => Self::ThreeOfAKind,
            (_, _, _, 2) => Self::TwoPairs,
            (_, _, _, 1) => Self::OnePair,
            (_, _, _, 0) => Self::HighCard,
            (_, _, _, _) => unreachable!(),
        }
    }
}

struct CardHand {
    cards: Cards,
    bid: u32,
    strength: CardHandStrength,
}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength && self.cards == other.cards
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for CardHand {}

fn parse_card_hands(input: &str, joker: bool) -> Vec<CardHand> {
    input
        .lines()
        .map(|line| {
            let (cards_string, bid_string) = line.split_once(' ').unwrap();
            let mut cards: Cards = [0u8; 5];
            cards_string
                .as_bytes()
                .iter()
                .map(|b| match b {
                    b'A' => 14u8,
                    b'K' => 13u8,
                    b'Q' => 12u8,
                    b'J' => 1u8 + 10u8 * !joker as u8,
                    b'T' => 10u8,
                    _ => b - b'0',
                })
                .enumerate()
                .for_each(|(i, b)| cards[i] = b);
            CardHand {
                cards,
                bid: bid_string.parse::<u32>().unwrap(),
                strength: CardHandStrength::evaluate_strength(&cards, joker),
            }
        })
        .collect()
}

fn solve_part01(input: &str) -> u32 {
    let mut hands = parse_card_hands(input, false);
    hands.sort();

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u32)
        .sum::<u32>();

    total_winnings
}

fn solve_part02(input: &str) -> u32 {
    let mut hands = parse_card_hands(input, true);
    hands.sort();

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u32)
        .sum::<u32>();

    total_winnings
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
    fn day07_part01() {
        let input = include_str!("input_test.txt");
        let solution = solve_part01(input);
        assert_eq!(solution, 6440);
    }

    #[test]
    fn day07_part02() {
        let input = include_str!("input_test.txt");
        let solution = solve_part02(input);
        assert_eq!(solution, 5905);
    }
}
