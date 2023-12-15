fn calc_next_value(history: Vec<i64>) -> i64 {
    if history.iter().all(|&value| value == 0) {
        return 0;
    }
    let next_history = history
        .windows(2)
        .map(|values| values[1] - values[0])
        .collect::<Vec<_>>();
    history.last().unwrap() + calc_next_value(next_history)
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(calc_next_value)
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<_>>()
        })
        .map(calc_next_value)
        .sum::<i64>()
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
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 2);
    }
}
