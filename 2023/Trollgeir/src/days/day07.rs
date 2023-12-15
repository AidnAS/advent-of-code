use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    _type: HandType,
    strength: [u8; 5],
    bet: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_ordering = self._type.cmp(&other._type);
        if type_ordering == std::cmp::Ordering::Equal {
            self.strength.cmp(&other.strength)
        } else {
            type_ordering
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn from_str(s: &str, joker: bool) -> Result<Self, &'static str> {
        let (hand_str, bet_str) = s.split_whitespace().collect_tuple().expect("2 parts");
        let bet = bet_str.parse().expect("Bet can be parsed");

        let mut strength: [u8; 5] = [0; 5];
        hand_str
            .chars()
            .map(|c| match c {
                'A' => b'a',
                'K' => b'b',
                'Q' => b'c',
                'J' => match joker {
                    true => b'n',
                    false => b'd',
                },
                'T' => b'e',
                '9' => b'f',
                '8' => b'g',
                '7' => b'h',
                '6' => b'i',
                '5' => b'j',
                '4' => b'k',
                '3' => b'l',
                '2' => b'm',
                _ => panic!("Invalid card label"),
            })
            .enumerate()
            .for_each(|(i, s)| strength[i] = s);

        let mut pairs = 0;
        let mut three = false;
        let mut four = false;
        let mut five = false;
        let mut counts: [u8; 14] = [0; 14];
        for &c in strength.iter() {
            counts[c as usize - b'a' as usize] += 1;
        }

        let card_types_to_count = if joker { 13 } else { 14 };
        counts
            .iter()
            .take(card_types_to_count)
            .for_each(|&count| match count {
                2 => pairs += 1,
                3 => three = true,
                4 => four = true,
                5.. => five = true,
                _ => {}
            });

        if joker {
            let num_jokers = counts[13];

            match (num_jokers, four, three, pairs) {
                (1.., true, _, _) => five = true,
                (1, _, true, _) => four = true,
                (2, _, true, _) => five = true,
                (1.., _, _, 1..) => {
                    // Mutate pairs into something else with a joker
                    match (num_jokers, pairs) {
                        (2, 1) => four = true,
                        (3, 1) => five = true,
                        _ => three = true,
                    }
                    pairs -= 1;
                }
                (1, _, _, 0) => pairs += 1,
                (2, _, _, 0) => three = true,
                (3, _, _, 0) => four = true,
                (4.., _, _, 0) => five = true,
                _ => {}
            }
        }

        let hand_type = match (five, four, three, pairs) {
            (true, _, _, _) => HandType::FiveOfAKind,
            (_, true, _, _) => HandType::FourOfAKind,
            (_, _, true, 1) => HandType::FullHouse,
            (_, _, true, _) => HandType::ThreeOfAKind,
            (_, _, _, 2) => HandType::TwoPair,
            (_, _, _, 1) => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Ok(Hand {
            _type: hand_type,
            strength,
            bet,
        })
    }
}

pub fn solve() -> SolutionPair {
    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let bets_sum_1 = reader
        .lines()
        .map(|line| Hand::from_str(line.unwrap().as_str(), false))
        .sorted()
        .rev()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u32 * hand.unwrap().bet)
        .sum();

    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let bets_sum_2 = reader
        .lines()
        .map(|line| Hand::from_str(line.unwrap().as_str(), true))
        .sorted()
        .rev()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u32 * hand.unwrap().bet)
        .sum();

    (Solution::U32(bets_sum_1), Solution::U32(bets_sum_2))
}
