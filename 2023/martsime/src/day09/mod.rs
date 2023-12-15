use crate::solution::Solution;

fn parse_line(line: &str) -> Vec<Vec<i32>> {
    let numbers = line
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut layers = vec![numbers];
    let mut current_layer = &layers[0];
    while !current_layer.iter().all(|&number| number == 0) {
        let next_layer = current_layer
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        layers.push(next_layer);
        current_layer = layers.last().as_ref().unwrap();
    }
    layers
}

fn solve_part01(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let layers = parse_line(line);
            layers
                .iter()
                .rev()
                .map(|layer| layer.last().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn solve_part02(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let layers = parse_line(line);
            layers
                .iter()
                .rev()
                .map(|layer| layer.first().unwrap())
                .fold(0, |acc, number| number - acc)
        })
        .sum::<i32>()
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
    fn day09_part01() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve_part01(input), 114);
    }
    #[test]
    fn day09_part02() {
        let input = include_str!("input_test.txt");
        assert_eq!(solve_part02(input), 2);
    }
}
