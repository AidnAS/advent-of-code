use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat_with;

#[derive(Debug)]
struct MapRule {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

#[derive(Debug)]
struct Map {
    maps: Vec<MapRule>,
}
impl Map {
    fn map(&self, value: u32) -> u32 {
        for rule in &self.maps {
            if value >= rule.source_range_start
                && value < rule.source_range_start + rule.range_length
            {
                return value - rule.source_range_start + rule.destination_range_start;
            }
        }
        value
    }
}

fn try_parse_line_into_rule(line: &str) -> Option<MapRule> {
    line.split_whitespace()
        .flat_map(str::parse)
        .collect_tuple::<(u32, u32, u32)>()
        .map(|(a, b, c)| MapRule {
            destination_range_start: a,
            source_range_start: b,
            range_length: c,
        })
}

// Function to process lines until an empty line is encountered
fn get_next_map<I>(lines: &mut dyn Iterator<Item = Result<String, I>>) -> Option<Map> {
    for line in &mut *lines {
        let line = line.ok()?;

        if line.contains("map:") {
            let mut map = Map { maps: Vec::new() };
            for result_line in lines.take_while(|result| {
                result
                    .as_ref()
                    .map_or(false, |line| !line.trim().is_empty())
            }) {
                let line = result_line.ok()?;
                let map_rule = try_parse_line_into_rule(&line)?;
                map.maps.push(map_rule);
            }

            return Some(map);
        }
    }
    None
}

pub fn solve() -> SolutionPair {
    let reader = BufReader::new(File::open("input/input.txt").unwrap());
    let mut lines = reader.lines();

    let seeds: Vec<u32> = lines
        .next()
        .and_then(|line| line.unwrap().rsplit(':').next().map(String::from))
        .map(|s| s.split_whitespace().flat_map(str::parse).collect())
        .unwrap();

    let maps: Vec<Map> = repeat_with(|| get_next_map(&mut lines))
        .take_while(Option::is_some)
        .map(Option::unwrap)
        .collect();

    // Part 1
    let min_location = seeds
        .iter()
        .map(|s| maps.iter().fold(*s, |output, map| map.map(output)))
        .min()
        .unwrap();

    // Part 2 ... yeah.. (:
    let min_location_2 = seeds
        .windows(2)
        .step_by(2)
        .flat_map(|window| {
            let (a, b) = (window[0], window[1]);
            a..a + b
        })
        .map(|s| maps.iter().fold(s, |output, map| map.map(output)))
        .min()
        .unwrap();

    (Solution::U32(min_location), Solution::U32(min_location_2))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mapper() {
        let rule1 = MapRule {
            destination_range_start: 52,
            source_range_start: 50,
            range_length: 48,
        };
        let rule2 = MapRule {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };

        let mapper = Map {
            maps: vec![rule1, rule2],
        };

        assert_eq!(mapper.map(0), 0);
        assert_eq!(mapper.map(48), 48);
        assert_eq!(mapper.map(49), 49);
        assert_eq!(mapper.map(50), 52);
        assert_eq!(mapper.map(51), 53);
        assert_eq!(mapper.map(97), 99);
        assert_eq!(mapper.map(98), 50);
        assert_eq!(mapper.map(99), 51);
    }
}
