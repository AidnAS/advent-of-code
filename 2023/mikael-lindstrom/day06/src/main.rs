fn part1(input: &str) -> u32 {
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    let time = time_str
        .trim_start_matches("Time:")
        .split_whitespace()
        .collect::<Vec<_>>();
    let distance = distance_str
        .trim_start_matches("Distance:")
        .split_whitespace()
        .collect::<Vec<_>>();

    time.iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, time)| {
            acc.push((
                time.parse::<u32>().unwrap(),
                distance[i].parse::<u32>().unwrap(),
            ));
            acc
        })
        .iter()
        .map(|(time, distance)| {
            let min = (1..*time)
                .find(|press| press * (time - press) > *distance)
                .unwrap();
            let max = (1..*time)
                .rev()
                .find(|press| press * (time - press) > *distance)
                .unwrap();
            max - min + 1
        })
        .product::<u32>()
}

fn part2(input: &str) -> u64 {
    let (time_str, distance_str) = input.split_once('\n').unwrap();
    let time = time_str
        .trim_start_matches("Time:")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance = distance_str
        .trim_start_matches("Distance:")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let min = (1..time)
        .find(|press| press * (time - press) > distance)
        .unwrap();
    let max = (1..time)
        .rev()
        .find(|press| press * (time - press) > distance)
        .unwrap();
    max - min + 1
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
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 71503);
    }
}
