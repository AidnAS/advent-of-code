use rayon::prelude::*;

use crate::solution::Solution;
use crate::utils::parse_number_from_bytes;

pub struct Cache {
    map: Vec<i64>,
    springs: usize,
}

impl Cache {
    pub fn new(springs: usize, groups: usize) -> Self {
        Self {
            map: vec![-1; (springs + 1) * (groups + 1)],
            springs: springs + 1,
        }
    }

    #[inline]
    pub fn get(&self, springs: &[u8], groups: &[usize]) -> Option<u64> {
        match self.map[groups.len() * self.springs + springs.len()] {
            -1 => None,
            value => Some(value as u64),
        }
    }

    #[inline]
    pub fn get_and_set(&mut self, springs: &[u8], groups: &[usize], value: u64) -> u64 {
        self.map[groups.len() * self.springs + springs.len()] = value as i64;
        // self.map.insert((springs, groups), value);
        value
    }
}

fn parse(input: &[u8], repetitions: usize) -> Vec<(Vec<u8>, Vec<usize>)> {
    input
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut line_iter = line.split(|&b| b == b' ');
            let springs_slice = line_iter.next().unwrap();
            let num_springs = springs_slice.len() * repetitions + repetitions - 1;
            let springs = springs_slice
                .iter()
                .chain([b'?'].iter())
                .copied()
                .cycle()
                .take(num_springs)
                .collect::<Vec<_>>();
            let groups = line_iter
                .next()
                .map(|s| {
                    let groups = s
                        .split(|&b| b == b',')
                        .map(parse_number_from_bytes::<usize>)
                        .collect::<Vec<_>>();

                    let num_groups = groups.len() * repetitions;

                    groups
                        .into_iter()
                        .cycle()
                        .take(num_groups)
                        .collect::<Vec<_>>()
                })
                .unwrap();
            (springs, groups)
        })
        .collect::<Vec<_>>()
}

#[inline]
fn match_and_return_next(springs: &[u8], group: usize) -> Option<&[u8]> {
    // If there is any working springs -> this cannot be a group
    if springs[..group].iter().any(|&b| b == b'.') {
        return None;
    }

    // Check the next spring after the group
    match springs.get(group).copied().unwrap_or(b'.') {
        // If the next spring is working, the group is not valid
        b'#' => None,
        // Otherwise, return the remaining springs
        _ => match springs.len() == group {
            true => Some(&[]),
            false => Some(&springs[group + 1..]),
        },
    }
}

#[inline]
fn arrangements(mut springs: &[u8], groups: &[usize], cache: &mut Cache) -> u64 {
    while let Some(first) = springs.first() {
        if first == &b'.' {
            springs = &springs[1..];
        } else {
            break;
        }
    }

    // No springs left -> a possible arrangement only if no groups left
    if springs.is_empty() {
        return groups.is_empty() as u64;
    }

    // No groups left -> no arrangement if damaged springs left
    if groups.is_empty() {
        return !springs.iter().any(|b| b == &b'#') as u64;
    }

    // Check cache
    if let Some(value) = cache.get(springs, groups) {
        return value;
    }

    // Not enough springs left for the remaining groups -> no arrangements
    if springs.len() < groups.iter().sum::<usize>() + groups.len() - 1 {
        return cache.get_and_set(springs, groups, 0);
    }

    // If the first spring is unknown, we test both cases
    if springs[0] == b'?' {
        // First case: Assume spring is working -> skip it
        let a = arrangements(&springs[1..], groups, cache);

        // Second case: Assume spring is damaged -> match next group
        let b = match match_and_return_next(springs, groups[0]) {
            Some(next_group) => arrangements(next_group, &groups[1..], cache),
            None => 0,
        };
        return cache.get_and_set(springs, groups, a + b);
    }

    // Next spring is damaged, match the group and possibly continue with the rest
    let value = match match_and_return_next(springs, groups[0]) {
        Some(next_group) => arrangements(next_group, &groups[1..], cache),
        None => 0,
    };

    cache.get_and_set(springs, groups, value)
}

fn solve_part01(input: &[u8]) -> u64 {
    parse(input, 1)
        .par_iter()
        .map(|(springs, groups)| {
            arrangements(
                springs,
                groups,
                &mut Cache::new(springs.len(), groups.len()),
            )
        })
        .sum()
}

fn solve_part02(input: &[u8]) -> u64 {
    parse(input, 5)
        .par_iter()
        .map(|(springs, groups)| {
            arrangements(
                springs,
                groups,
                &mut Cache::new(springs.len(), groups.len()),
            )
        })
        .sum()
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
    fn day12_part01() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part01(input), 21);
    }

    #[test]
    fn day12_part02() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part02(input), 525152);
    }
}
