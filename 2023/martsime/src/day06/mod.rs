use crate::solution::Solution;

fn parse_line(line: &'_ str) -> impl Iterator<Item = u32> + '_ {
    line.split(':')
        .nth(1)
        .map(|numbers| {
            numbers
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
        })
        .unwrap()
}

fn parse_part01(input: &'_ str) -> impl Iterator<Item = (u32, u32)> + '_ {
    let (first, second) = input.split_once('\n').unwrap();
    parse_line(first)
        .zip(parse_line(second))
        .map(|(a, b)| (a, b))
}

fn parse_part02(input: &str) -> (u64, u64) {
    let mut numbers = input.lines().map(|line| {
        line.trim_start_matches("Time:")
            .trim_start_matches("Distance:")
            .replace(' ', "")
            .parse::<u64>()
            .unwrap()
    });

    (numbers.next().unwrap(), numbers.next().unwrap())
}

fn solve_part01(input: &str) -> u32 {
    let mut total = 1;
    for (time, record) in parse_part01(input) {
        let mut better_ways = 0;
        for hold_time in 1..time {
            let drive_time = time - hold_time;
            if drive_time * hold_time > record {
                better_ways += 1;
            }
        }
        total *= better_ways;
    }
    total
}

fn solve_part02(input: &str) -> u64 {
    // The inequality we want to solve is:
    // x * (time - x) > record
    // Rewritten as:
    // x^2 - time * x + record < 0
    // This is a quadratic inequality, in which the left hand is an upside down parabola, meaning
    // every point between the solutions to the quadratic equality satisfies the inequality
    // x = (-b +- sqrt(b^2 - 4ac)) / 2a
    let (time, record) = parse_part02(input);

    let a = 1f64;
    let b = -(time as f64);
    let c = record as f64;

    let x1 = (-b + (b * b - 4f64 * a * c).sqrt()) / (2f64 * a);
    let x2 = (-b - (b * b - 4f64 * a * c).sqrt()) / (2f64 * a);

    let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    x2.floor() as u64 - x1.ceil() as u64 + 1
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
    fn day06_part01() {
        let input = include_str!("input_test.txt");
        let solution = solve_part01(input);
        assert_eq!(solution, 288);
    }

    #[test]
    fn day06_part02() {
        let input = include_str!("input_test.txt");
        let solution = solve_part02(input);
        assert_eq!(solution, 71503);
    }
}
