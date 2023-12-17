use std::collections::HashMap;

use crate::solution::Solution;

type IndexType = i32;
type Cost = u32;
type Position = (IndexType, IndexType);
type Direction = (IndexType, IndexType);
type Ray = (Position, Direction);
type Node = (Cost, Ray);

const UP: Direction = (0, -1);
const DOWN: Direction = (0, 1);
const RIGHT: Direction = (1, 0);
const LEFT: Direction = (-1, 0);

const DIRECTIONS: [Direction; 4] = [RIGHT, DOWN, LEFT, UP];

struct Board<'a> {
    bytes: &'a [u8],
    width: IndexType,
    height: IndexType,
}

impl<'a> Board<'a> {
    pub fn parse(input: &'a [u8]) -> Self {
        let width = input.iter().position(|&b| b == b'\n').unwrap() as IndexType;
        let height = input.len() as IndexType / (width);

        Self {
            bytes: input,
            width,
            height,
        }
    }

    #[inline]
    pub fn get(&self, x: IndexType, y: IndexType) -> u8 {
        self.bytes[(y * (self.width + 1) + x) as usize] - b'0'
    }
}

fn shortest_path(
    board: &Board,
    start: Position,
    end: Position,
    min_steps: IndexType,
    max_steps: IndexType,
) -> Cost {
    let mut distances: HashMap<(Position, Direction), Cost> = HashMap::new();
    let mut queue: Vec<Node> = vec![(0, (start, (0, 0)))];
    while let Some((cost, (position, previous_direction))) = queue.pop() {
        if position == end {
            return cost;
        }

        if let Some(&cheapest_cost) = distances.get(&(position, previous_direction)) {
            if cheapest_cost < cost {
                continue;
            }
        }

        for (dx, dy) in DIRECTIONS {
            if previous_direction == (dx, dy) || previous_direction == (-dx, -dy) {
                continue;
            }

            let mut next_cost = cost;

            for length in 1..=max_steps {
                let (x, y) = position;
                let (nx, ny) = (x + dx * length, y + dy * length);
                if nx < 0 || nx >= board.width || ny < 0 || ny >= board.height {
                    break;
                }

                next_cost += board.get(nx, ny) as Cost;

                let key = ((nx, ny), (dx, dy));
                if length >= min_steps && next_cost < *distances.get(&key).unwrap_or(&1_000_000) {
                    distances.insert(key, next_cost);
                    queue.push((next_cost, key));
                }
            }
        }

        queue.sort_by(|a, b| b.0.cmp(&a.0));
    }

    unreachable!()
}

fn solve_part01(input: &[u8]) -> u32 {
    let board = Board::parse(input);
    shortest_path(&board, (0, 0), (board.width - 1, board.height - 1), 1, 3)
}

fn solve_part02(input: &[u8]) -> u32 {
    let board = Board::parse(input);
    shortest_path(&board, (0, 0), (board.width - 1, board.height - 1), 4, 10)
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
    fn day17_part01() {
        let input = include_bytes!("input_test1.txt");
        assert_eq!(solve_part01(input), 102);
    }

    #[test]
    fn day17_part02_1() {
        let input = include_bytes!("input_test1.txt");
        assert_eq!(solve_part02(input), 94);
    }

    #[test]
    fn day17_part02_2() {
        let input = include_bytes!("input_test2.txt");
        assert_eq!(solve_part02(input), 71);
    }
}
