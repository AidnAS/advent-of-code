use std::ops::Add;

fn part1(input: &str) -> i64 {
    let mut input = input.split("\n\n");
    let seeds = input
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let maps = input
        .map(|map| {
            map.lines().skip(1).fold(Vec::new(), |mut acc, line| {
                let n = line
                    .split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                acc.push((n[1]..n[1] + n[2], n[0] - n[1]));
                acc
            })
        })
        .collect::<Vec<_>>();

    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(seed.to_owned(), |acc, map| {
                map.iter()
                    .find_map(|range| {
                        if range.0.contains(&acc) {
                            return Some(acc.saturating_add(range.1));
                        }
                        None
                    })
                    .unwrap_or(acc)
            })
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let mut input = input.split("\n\n");
    let seeds = input
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split(' ')
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| {
            let start = x[0].parse::<i64>().unwrap();
            let length = x[1].parse::<i64>().unwrap();
            start..=start + length - 1
        })
        .collect::<Vec<_>>();

    let maps = input
        .map(|map| {
            map.lines().skip(1).fold(Vec::new(), |mut acc, line| {
                let n = line
                    .split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                acc.push((n[1]..=n[1] + n[2], n[0] - n[1]));
                acc
            })
        })
        .collect::<Vec<_>>();

    maps.iter()
        .fold(seeds, |seeds, map| {
            seeds.iter().fold(Vec::new(), |mut acc, seed| {
                let mut remaining = Vec::new();
                for range in map.iter() {
                    match (range.0.contains(seed.start()), range.0.contains(seed.end())) {
                        (true, true) => {
                            acc.push(seed.start().add(range.1)..=seed.end().add(range.1));
                        }
                        (true, false) => {
                            remaining.push(seed.start().to_owned()..=range.0.end().to_owned() - 1);
                            acc.push(seed.start().add(range.1)..=range.0.end().add(range.1) - 1);
                        }
                        (false, true) => {
                            remaining.push(range.0.start().to_owned()..=seed.end().to_owned());
                            acc.push(range.0.start().add(range.1)..=seed.end().add(range.1));
                        }
                        _ => {}
                    }
                }
                if acc.is_empty() {
                    acc.push(seed.start().to_owned()..=seed.end().to_owned());
                } else if !remaining.is_empty() {
                    remaining.sort_by(|a, b| a.start().cmp(b.start()));
                    // Currently only handles if seed start and/or end is outside the map
                    match (
                        *seed.start() < *remaining.first().unwrap().start(),
                        *seed.end() > *remaining.last().unwrap().end(),
                    ) {
                        (true, false) => {
                            acc.push(
                                seed.start().to_owned()
                                    ..=remaining.first().unwrap().start().to_owned() - 1,
                            );
                        }
                        (false, true) => {
                            acc.push(
                                remaining.last().unwrap().end().to_owned() + 1
                                    ..=seed.end().to_owned(),
                            );
                        }
                        _ => {}
                    }
                }
                acc
            })
        })
        .iter()
        .map(|seed_range| seed_range.start().to_owned())
        .min()
        .unwrap()
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
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 46);
    }
}
