use crate::solution::Solution;

enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone)]
struct Board {
    bytes: Vec<u8>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn parse(input: &[u8]) -> Self {
        let width = input.iter().position(|&b| b == b'\n').unwrap();
        let height = input.len() / (width);
        let bytes = input
            .iter()
            .filter(|&&b| b != b'\n')
            .copied()
            .collect::<Vec<_>>();

        Self {
            bytes,
            width,
            height,
        }
    }

    #[inline]
    pub fn cycle(&mut self, directions: &[Direction]) {
        for direction in directions {
            match direction {
                Direction::North => {
                    for x in 0..self.width {
                        let mut empty_y = 0;
                        for y in 0..self.height {
                            match self.bytes[y * self.width + x] {
                                b'O' => {
                                    if y != empty_y {
                                        self.bytes[empty_y * self.width + x] = b'O';
                                        self.bytes[y * self.width + x] = b'.';
                                    }
                                    empty_y += 1;
                                }
                                b'#' => empty_y = y + 1,
                                _ => {}
                            }
                        }
                    }
                }
                Direction::West => {
                    for y in 0..self.height {
                        let mut empty_x = 0;
                        for x in 0..self.width {
                            match self.bytes[y * self.width + x] {
                                b'O' => {
                                    if x != empty_x {
                                        self.bytes[y * self.width + empty_x] = b'O';
                                        self.bytes[y * self.width + x] = b'.';
                                    }
                                    empty_x += 1;
                                }
                                b'#' => empty_x = x + 1,
                                _ => {}
                            }
                        }
                    }
                }
                Direction::South => {
                    for x in 0..self.width {
                        let mut empty_y = self.height - 1;
                        for y in (0..self.height).rev() {
                            match self.bytes[y * self.width + x] {
                                b'O' => {
                                    if y != empty_y {
                                        self.bytes[empty_y * self.width + x] = b'O';
                                        self.bytes[y * self.width + x] = b'.';
                                    }
                                    empty_y = empty_y.saturating_sub(1);
                                }
                                b'#' => empty_y = y.saturating_sub(1),
                                _ => {}
                            }
                        }
                    }
                }
                Direction::East => {
                    for y in 0..self.height {
                        let mut empty_x = self.width - 1;
                        for x in (0..self.width).rev() {
                            match self.bytes[y * self.width + x] {
                                b'O' => {
                                    if x != empty_x {
                                        self.bytes[y * self.width + empty_x] = b'O';
                                        self.bytes[y * self.width + x] = b'.';
                                    }
                                    empty_x = empty_x.saturating_sub(1);
                                }
                                b'#' => empty_x = x.saturating_sub(1),
                                _ => {}
                            }
                        }
                    }
                }
            };
        }
    }

    pub fn calculate_load(&self) -> u64 {
        let mut load = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.bytes[y * self.width + x] == b'O' {
                    load += self.height - y;
                }
            }
        }
        load as u64
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.bytes[y * self.width + x] as char);
            }
            println!();
        }
        println!();
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        (0..self.bytes.len()).all(|i| self.bytes[i] == other.bytes[i])
    }
}

fn solve_part01(input: &[u8]) -> u64 {
    let mut board = Board::parse(input);
    board.cycle(&[Direction::North]);
    board.calculate_load()
}

fn solve_part02(input: &[u8]) -> u64 {
    let mut board = Board::parse(input);
    let mut board_cycles: Vec<Board> = Vec::with_capacity(1000);
    board_cycles.push(board.clone());
    let total_iterations = 1_000_000_000;
    let directions = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    for iteration in 1..total_iterations {
        board.cycle(&directions);
        let interval =
            board_cycles.iter().rev().enumerate().find_map(
                |(interval, prev_board)| match prev_board == &board {
                    true => Some(interval + 1),
                    false => None,
                },
            );

        if let Some(interval) = interval {
            let correct_iteration =
                iteration + ((total_iterations - iteration) % interval) - interval;
            return board_cycles[correct_iteration].calculate_load();
        } else {
            board_cycles.push(board.clone());
        }
    }

    panic!("No solution found");
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
    fn day14_part01() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part01(input), 136);
    }

    #[test]
    fn day14_part02() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part02(input), 64);
    }
}
