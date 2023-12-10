use itertools::Itertools;

use crate::{Solution, SolutionPair};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Rule = (char, char, char);
enum Direction {
    Right,
    Left,
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn find_num_steps_to_z(
    start_node: Rule,
    instructions: &[Direction],
    rule_map: &HashMap<Rule, (Rule, Rule)>,
) -> Option<usize> {
    let mut current_node = start_node;
    instructions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(i, direction)| {
            if current_node.2 == 'Z' {
                return Some(i);
            }
            let (left, right) = rule_map.get(&current_node).unwrap();
            let rule = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };
            current_node.0 = rule.0;
            current_node.1 = rule.1;
            current_node.2 = rule.2;
            None
        })
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'R' => Some(Self::Right),
            'L' => Some(Self::Left),
            _ => None,
        }
    }
}
pub fn solve() -> SolutionPair {
    let pattern: Regex = Regex::new(r"([A-Z0-9]{3})").unwrap();
    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let mut lines = reader.lines();

    let instructions: Vec<Direction> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .filter_map(Direction::from_char)
        .collect();

    let rule_map = lines
        .skip(1)
        .map(|line| {
            let line = line.unwrap();
            let (lookup, left, right) = pattern
                .captures_iter(&line)
                .map(|cap| cap[0].chars().collect_tuple::<Rule>().unwrap())
                .collect_tuple()
                .unwrap();
            (lookup, (left, right))
        })
        .collect::<HashMap<Rule, (Rule, Rule)>>();

    // Part 1
    let start_node = ('A', 'A', 'A');
    let steps_1 = find_num_steps_to_z(start_node, &instructions, &rule_map).unwrap();

    // Part 2
    let start_nodes = rule_map
        .iter()
        .filter(|(rule, _)| rule.2 == 'A')
        .map(|(rule, _)| *rule)
        .collect::<Vec<_>>();

    let z_intervals: Vec<usize> = start_nodes
        .into_iter()
        .map(|node| find_num_steps_to_z(node, &instructions, &rule_map).unwrap())
        .collect();

    let steps_2 = z_intervals.iter().copied().reduce(lcm).unwrap();

    (Solution::U64(steps_1 as u64), Solution::U64(steps_2 as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm() {
        let z_intervals = [2, 3, 4, 5];
        assert_eq!(z_intervals.iter().copied().reduce(lcm), Some(60));

        let z_intervals = [2, 3];
        assert_eq!(z_intervals.iter().copied().reduce(lcm), Some(6));
    }
}
