use crate::solution::Solution;

use num::integer::lcm;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// Assumes key range [0-9A-Z]
const KEY_LENGTH: usize = 3;
const KEY_RANGE: usize = 36;
const BUFFER_SIZE: usize = KEY_RANGE.pow(KEY_LENGTH as u32);

#[derive(Clone, Copy, Default)]
struct Node {
    bytes: [u8; KEY_LENGTH],
    index: u16,
}

impl Node {
    #[inline]
    fn new(bytes: &[u8]) -> Self {
        let bytes: [u8; KEY_LENGTH] = bytes.try_into().unwrap();
        Self {
            bytes,
            index: calculate_index(bytes) as u16,
        }
    }
}

#[inline(always)]
fn calculate_index(bytes: [u8; KEY_LENGTH]) -> usize {
    bytes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let b = if *b <= b'9' { b - b'0' } else { b - b'A' + 10 };
            b as usize * KEY_RANGE.pow((KEY_LENGTH - i - 1) as u32)
        })
        .sum()
}

#[derive(Clone, Copy, Default)]
struct Connection {
    left: Node,
    right: Node,
}

fn solve_part01(input: &[u8]) -> u32 {
    let mut line_iter = input.split(|&b| b == b'\n');
    let directions = line_iter.next().unwrap();
    line_iter.next(); // Skip empty line

    let mut connections: [Connection; BUFFER_SIZE] = [Default::default(); BUFFER_SIZE];

    line_iter.for_each(|line| {
        let from = Node::new(&line[0..3]);
        let left = Node::new(&line[7..10]);
        let right = Node::new(&line[12..15]);
        connections[from.index as usize] = Connection { left, right };
    });

    let mut current_node = Node::new(&[b'A', b'A', b'A']);
    let mut steps = 0;

    'outer: loop {
        for dir in directions {
            steps += 1;
            current_node = match dir {
                b'L' => connections[current_node.index as usize].left,
                b'R' => connections[current_node.index as usize].right,
                _ => unreachable!(),
            };

            if current_node.bytes == [b'Z', b'Z', b'Z'] {
                break 'outer;
            }
        }
    }

    steps
}

fn solve_part02(input: &[u8]) -> u64 {
    let mut line_iter = input.split(|&b| b == b'\n');
    let directions = line_iter.next().unwrap();
    line_iter.next(); // Skip empty line

    let mut connections: [Connection; BUFFER_SIZE] = [Default::default(); BUFFER_SIZE];

    let start_nodes = line_iter
        .filter_map(|line| {
            let from = Node::new(&line[0..3]);
            let left = Node::new(&line[7..10]);
            let right = Node::new(&line[12..15]);
            connections[from.index as usize] = Connection { left, right };

            if from.bytes[2] == b'A' {
                Some(from)
            } else {
                None
            }
        })
        .collect::<Vec<Node>>();

    start_nodes
        .par_iter()
        .map(|start_node| {
            let mut current_node = start_node;
            let mut steps = 0;
            'outer: loop {
                for dir in directions {
                    steps += 1;
                    current_node = match dir {
                        b'L' => &connections[current_node.index as usize].left,
                        b'R' => &connections[current_node.index as usize].right,
                        _ => unreachable!(),
                    };
                    if current_node.bytes[2] == b'Z' {
                        break 'outer;
                    }
                }
            }
            steps
        })
        .reduce(|| 1, lcm)
}

pub fn part01() -> Solution {
    solve_part01(include_bytes!("input.txt")).into()
}

pub fn part02() -> Solution {
    solve_part02(include_bytes!("input.txt")).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_part01_1() {
        let input = include_bytes!("input_test01.txt");
        assert_eq!(solve_part01(input), 2);
    }
    #[test]
    fn day08_part01_2() {
        let input = include_bytes!("input_test02.txt");
        assert_eq!(solve_part01(input), 6);
    }
    #[test]
    fn day08_part02() {
        let input = include_bytes!("input_test03.txt");
        assert_eq!(solve_part02(input), 6);
    }

    #[test]
    fn test_calculate_index() {
        assert_eq!(calculate_index([b'0', b'0', b'0']), 0);
        assert_eq!(calculate_index([b'0', b'0', b'1']), 1);
        assert_eq!(
            calculate_index([b'A', b'A', b'B']),
            36 * 36 * 10 + 36 * 10 + 11
        );
        assert_eq!(calculate_index([b'Z', b'Z', b'Z']), 36 * 36 * 36 - 1);
        assert_eq!(calculate_index([b'Z', b'Z', b'Z']), 36 * 36 * 36 - 1);
    }
}
