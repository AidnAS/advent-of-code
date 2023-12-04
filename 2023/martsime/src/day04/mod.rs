use crate::solution::Solution;

struct Buffer<const N: usize, T> {
    buffer: [T; N],
    index: usize,
}

impl<const N: usize, T: Default + Copy + Eq> Buffer<N, T> {
    #[inline(always)]
    fn new() -> Self {
        Self {
            buffer: [Default::default(); N],
            index: 0,
        }
    }

    #[inline(always)]
    fn insert(&mut self, value: T) {
        self.buffer[self.index] = value;
        self.index += 1;
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.index = 0;
    }

    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter().take(self.index)
    }

    #[inline(always)]
    fn slice(&self) -> &[T] {
        &self.buffer[..self.index]
    }

    #[inline(always)]
    fn intersection(&self, other: &Self) -> usize {
        self.slice()
            .iter()
            .filter(|&n| other.iter().any(|m| n == m))
            .count()
    }
}

type Number = [u8; 2];

fn solve_part01(input: &[u8]) -> u32 {
    const BUFFER_SIZE: usize = 100;
    let mut winning_numbers: Buffer<BUFFER_SIZE, Number> = Buffer::new();
    let mut numbers: Buffer<BUFFER_SIZE, Number> = Buffer::new();
    input
        .split(|&b| b == b'\n')
        .map(|line| {
            winning_numbers.clear();
            numbers.clear();
            let mut index = line.iter().position(|b| b == &b':').unwrap() + 2;
            let length = line.len();
            let mut first_buffer = true;
            while index < length {
                if line[index] == b'|' {
                    index += 2;
                    first_buffer = false;
                    continue;
                }
                if first_buffer {
                    winning_numbers.insert([line[index], line[index + 1]]);
                    index += 3;
                } else {
                    numbers.insert([line[index], line[index + 1]]);
                    index += 3;
                }
            }
            let numbers_matching = winning_numbers.intersection(&numbers) as u32;
            match numbers_matching {
                0 => 0,
                _ => 2u32.pow(numbers_matching - 1),
            }
        })
        .sum::<u32>()
}

fn solve_part02(input: &[u8]) -> u32 {
    const BUFFER_SIZE: usize = 100;
    let mut winning_numbers: Buffer<BUFFER_SIZE, Number> = Buffer::new();
    let mut numbers: Buffer<BUFFER_SIZE, Number> = Buffer::new();
    let mut copies = [1u32; 1_000];
    input
        .split(|&b| b == b'\n')
        .enumerate()
        .map(|(card_index, line)| {
            winning_numbers.clear();
            numbers.clear();
            let mut index = line.iter().position(|b| b == &b':').unwrap() + 2;
            let length = line.len();
            let mut first_buffer = true;
            while index < length {
                if line[index] == b'|' {
                    index += 2;
                    first_buffer = false;
                    continue;
                }
                if first_buffer {
                    winning_numbers.insert([line[index], line[index + 1]]);
                    index += 3;
                } else {
                    numbers.insert([line[index], line[index + 1]]);
                    index += 3;
                }
            }
            let numbers_matching = winning_numbers.intersection(&numbers);
            let card_copies = copies[card_index];

            (card_index + 1..=(card_index + numbers_matching)).for_each(|i| {
                copies[i] += card_copies;
            });
            card_copies
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
    fn day04_part01() {
        let input = include_bytes!("input_test.txt");
        let solution = solve_part01(input);
        assert_eq!(solution, 13);
    }

    #[test]
    fn day04_part02() {
        let input = include_bytes!("input_test.txt");
        let solution = solve_part02(input);
        assert_eq!(solution, 30);
    }
}
