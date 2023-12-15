use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[inline]
fn derivative<const FORWARD: bool>(numbers: &[i32]) -> i32 {
    if numbers.iter().all(|&n| n == 0) {
        return 0;
    }
    let derivatives: Vec<i32> = numbers.iter().tuple_windows().map(|(a, b)| b - a).collect();
    if FORWARD {
        derivative::<true>(&derivatives) + derivatives.last().unwrap()
    } else {
        derivatives.first().unwrap() - derivative::<false>(&derivatives)
    }
}

pub fn solve() -> SolutionPair {
    // Part 1
    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let sum_1: i32 = reader
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            derivative::<true>(&numbers) + numbers.last().unwrap()
        })
        .sum();

    // Part 2
    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let sum_2: i32 = reader
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            numbers.first().unwrap() - derivative::<false>(&numbers)
        })
        .sum();

    (Solution::I32(sum_1), Solution::I32(sum_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let numbers = [10, 13, 16, 21, 30, 45];
        assert_eq!(numbers[0] - derivative::<false>(&numbers), 5);

        let numbers = [0, 3, 6, 9, 12, 15];
        assert_eq!(numbers[0] - derivative::<false>(&numbers), -3);

        let numbers = [1, 3, 6, 10, 15, 21];
        assert_eq!(numbers[0] - derivative::<false>(&numbers), 0);
    }
}
