use std::ops::RangeInclusive;

use crate::solution::Solution;

pub struct SeedMap {
    pub source: RangeInclusive<i64>,
    pub destination: RangeInclusive<i64>,
    pub delta: i64,
}

impl SeedMap {
    pub fn new(destination_start: i64, source_start: i64, length: i64) -> Self {
        Self {
            source: source_start..=(source_start + length - 1),
            destination: destination_start..=(destination_start + length),
            delta: destination_start - source_start,
        }
    }
}

pub struct SeedNode {
    pub range: RangeInclusive<i64>,
    pub level: usize,
}

fn parse(input: &str) -> (Vec<i64>, Vec<Vec<SeedMap>>) {
    let mut line_iter = input.lines();
    let seeds = line_iter
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    line_iter.next(); // skip the empty line

    let mut seed_maps = Vec::with_capacity(7);
    loop {
        let header = line_iter.next();
        if header.is_none() {
            break;
        }
        let mut seed_map = Vec::new();
        loop {
            let line = line_iter.next().unwrap_or("");
            if line.is_empty() {
                break;
            }
            let mut number_iter = line.split(' ').map(|s| s.parse::<i64>().unwrap());
            let destination_start = number_iter.next().unwrap();
            let source_start = number_iter.next().unwrap();
            let length = number_iter.next().unwrap();
            seed_map.push(SeedMap::new(destination_start, source_start, length));
        }
        seed_maps.push(seed_map);
    }
    (seeds, seed_maps)
}

fn solve_part01(input: &str) -> i64 {
    let (seeds, seed_maps) = parse(input);

    let mut lowest = i64::MAX;

    for seed in seeds {
        let mut number = seed;
        for seed_map in &seed_maps {
            for map in seed_map {
                if map.source.contains(&number) {
                    number += map.delta;
                    break;
                }
            }
        }
        lowest = lowest.min(number);
    }
    lowest
}

fn solve_part02(input: &str) -> i64 {
    let (seeds, seed_maps) = parse(input);

    let mut queue = seeds
        .chunks(2)
        .map(|tuple| SeedNode {
            range: tuple[0]..=tuple[0] + tuple[1] - 1,
            level: 0,
        })
        .collect::<Vec<_>>();

    let mut lowest = i64::MAX;

    while let Some(node) = queue.pop() {
        if node.level == seed_maps.len() {
            lowest = lowest.min(*node.range.start());
            continue;
        }
        let seed_map = &seed_maps[node.level];

        let mut mapped = false;
        for map in seed_map {
            // Check if the range is contained in the map source.
            match (
                map.source.contains(node.range.start()),
                map.source.contains(node.range.end()),
            ) {
                // The range is fully contained and we can map the whole range to the next level.
                (true, true) => {
                    mapped = true;
                    queue.push(SeedNode {
                        range: node.range.start() + map.delta..=node.range.end() + map.delta,
                        level: node.level + 1,
                    });
                }
                // The range is only partially contained and have to be split.
                // We map the containing part to the next level and retry the rest of the range at the same level.
                (true, false) => {
                    mapped = true;
                    queue.push(SeedNode {
                        range: node.range.start() + map.delta..=map.source.end() + map.delta,
                        level: node.level + 1,
                    });
                    queue.push(SeedNode {
                        range: map.source.end() + 1..=*node.range.end(),
                        level: node.level,
                    });
                }
                // The range is only partially contained and have to be split.
                // We map the containing part to the next level and retry the rest of the range at the same level.
                (false, true) => {
                    mapped = true;
                    queue.push({
                        SeedNode {
                            range: *map.source.start() + map.delta..=*node.range.end() + map.delta,
                            level: node.level + 1,
                        }
                    });
                    queue.push({
                        SeedNode {
                            range: *node.range.start()..=*map.source.start() - 1,
                            level: node.level,
                        }
                    });
                }
                (false, false) => {}
            }
        }
        // Found no source map for the range, so it's mapped directly to the next level.
        if !mapped {
            queue.push({
                SeedNode {
                    range: node.range,
                    level: node.level + 1,
                }
            })
        }
    }
    lowest
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
    fn day05_part01() {
        let input = include_str!("input_test.txt");
        let solution = solve_part01(input);
        assert_eq!(solution, 35);
    }

    #[test]
    fn day05_part02() {
        let input = include_str!("input_test.txt");
        let solution = solve_part02(input);
        assert_eq!(solution, 46);
    }
}
