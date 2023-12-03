use std::str::from_utf8;

use crate::solution::Solution;

fn solve_part01(input: &[u8]) -> u32 {
    input
        .split(|b| *b == b'\n')
        .map(|line| {
            let first = line.iter().find(|b| b.is_ascii_digit()).unwrap();
            let last = line.iter().rev().find(|b| b.is_ascii_digit()).unwrap();
            from_utf8(&[*first, *last]).unwrap().parse::<u32>().unwrap()
        })
        .sum::<u32>()
}

const NUMBER_LITERALS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn solve_part02(input: &[u8]) -> u32 {
    input
        .split(|b| *b == b'\n')
        .map(|line| {
            let first = line
                .iter()
                .enumerate()
                .find_map(|(byte_index, b)| {
                    if b.is_ascii_digit() {
                        return Some(*b);
                    }

                    for (literal_index, literal) in NUMBER_LITERALS.iter().enumerate() {
                        if line[byte_index..].starts_with(literal) {
                            return Some((literal_index as u8 + 1) + b'0');
                        }
                    }
                    None
                })
                .unwrap();
            let last = line
                .iter()
                .enumerate()
                .rev()
                .find_map(|(byte_index, b)| {
                    if b.is_ascii_digit() {
                        return Some(*b);
                    }

                    for (literal_index, literal) in NUMBER_LITERALS.iter().enumerate() {
                        if line[byte_index..].starts_with(literal) {
                            return Some((literal_index as u8 + 1) + b'0');
                        }
                    }
                    None
                })
                .unwrap();
            from_utf8(&[first, last]).unwrap().parse::<u32>().unwrap()
        })
        .sum::<u32>()
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
    fn day01_part01() {
        let input = include_bytes!("input_test01.txt");
        let solution = solve_part01(input);
        assert_eq!(solution, 142);
    }

    #[test]
    fn day01_part02() {
        let input = include_bytes!("input_test02.txt");
        let solution = solve_part02(input);
        assert_eq!(solution, 281);
    }
}
