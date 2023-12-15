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
            maps.iter().fold(*seed, |acc, map| {
                map.iter()
                    .find_map(|range| {
                        if range.0.contains(&acc) {
                            return Some(acc.add(range.1));
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

    let mut queue = seeds;
    for map in maps.iter() {
        let mut for_next_map = Vec::new();
        for (range, offset) in map {
            let mut for_next_range = Vec::new();
            while let Some(seed) = queue.pop() {
                match (range.contains(seed.start()), range.contains(seed.end())) {
                    (false, true) => {
                        for_next_range.push(*seed.start()..=*range.start() - 1);
                        for_next_map.push(range.start().add(offset)..=seed.end().add(offset));
                    }
                    (true, true) => {
                        for_next_map.push(seed.start().add(offset)..=seed.end().add(offset));
                    }
                    (true, false) => {
                        for_next_range.push(*range.end() + 1..=*seed.end());
                        for_next_map.push(seed.start().add(offset)..=range.end().add(offset));
                    }
                    _ => {
                        for_next_range.push(*seed.start()..=*seed.end());
                    }
                }
            }
            queue = for_next_range;
        }
        queue.append(&mut for_next_map);
    }
    queue
        .iter()
        .map(|seed_range| *seed_range.start())
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
