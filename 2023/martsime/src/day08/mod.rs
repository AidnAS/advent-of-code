use std::collections::HashMap;

use crate::solution::Solution;

use num::integer::lcm;

fn solve_part01(input: &str) -> u32 {
    let mut line_iter = input.lines();
    let directions = line_iter.next().unwrap();
    line_iter.next(); // Skip empty line

    let nodes = line_iter
        .map(|line| {
            let key = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (key, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut current_node = "AAA";
    let end_node = "ZZZ";
    let mut steps = 0;

    'outer: loop {
        for dir in directions.chars() {
            steps += 1;
            current_node = nodes
                .get(current_node)
                .map(|(left, right)| match dir {
                    'L' => left,
                    'R' => right,
                    _ => unreachable!(),
                })
                .unwrap();
            if current_node == end_node {
                break 'outer;
            }
        }
    }

    steps
}

fn solve_part02(input: &str) -> u64 {
    let mut line_iter = input.lines();
    let directions = line_iter.next().unwrap();
    line_iter.next(); // Skip empty line

    let nodes = line_iter
        .map(|line| {
            let key = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (key, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let start_nodes = nodes
        .keys()
        .filter_map(|&key| {
            if key.as_bytes()[2] == b'A' {
                Some(key)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut steps = vec![0u64; start_nodes.len()];

    for node_index in 0..start_nodes.len() {
        let mut current_node = start_nodes[node_index];
        'outer: loop {
            for dir in directions.chars() {
                steps[node_index] += 1;
                current_node = nodes
                    .get(current_node)
                    .map(|(left, right)| match dir {
                        'L' => left,
                        'R' => right,
                        _ => unreachable!(),
                    })
                    .unwrap();
                if current_node.as_bytes()[2] == b'Z' {
                    break 'outer;
                }
            }
        }
    }

    steps.iter().fold(1, |x, &y| lcm(x, y))
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
    fn day08_part01_1() {
        let input = include_str!("input_test01.txt");
        assert_eq!(solve_part01(input), 2);
    }
    #[test]
    fn day08_part01_2() {
        let input = include_str!("input_test02.txt");
        assert_eq!(solve_part01(input), 6);
    }
    #[test]
    fn day08_part02() {
        let input = include_str!("input_test03.txt");
        assert_eq!(solve_part02(input), 6);
    }
}
