use std::collections::BinaryHeap;

use crate::solution::Solution;

type IndexType = i16;
type Cost = i32;
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

#[allow(dead_code)]
struct DistanceCache {
    distances: Vec<Cost>,
    width: usize,
    height: usize,
    size: usize,
}

impl DistanceCache {
    pub fn new(board: &Board) -> Self {
        let width = board.width as usize;
        let height = board.height as usize;
        let size = width * height;
        let directions = 2;
        let distances = vec![1_000_000; size * directions];

        Self {
            distances,
            width,
            height,
            size,
        }
    }

    #[inline]
    pub fn get(&self, key: (Position, Direction)) -> Cost {
        self.distances[self.index(key)]
    }

    #[inline]
    fn index(&self, key: (Position, Direction)) -> usize {
        let (position, direction) = key;
        let (x, y) = position;
        let dir_index = match direction {
            UP | DOWN => 0,
            RIGHT | LEFT => 1,
            _ => unreachable!(),
        };
        dir_index * self.size + (y as usize * self.width + x as usize)
    }

    #[inline]
    pub fn get_mut(&mut self, key: (Position, Direction)) -> &mut Cost {
        let index = self.index(key);
        &mut self.distances[index]
    }
}

fn shortest_path(
    board: &Board,
    start: Position,
    end: Position,
    min_steps: IndexType,
    max_steps: IndexType,
) -> Cost {
    let mut distances = DistanceCache::new(board);
    let start_node = (0, (start, (0, 0)));
    let mut queue: BinaryHeap<Node> = BinaryHeap::with_capacity(100);
    queue.push(start_node);
    while let Some((cost, (position, previous_direction))) = queue.pop() {
        let (x, y) = position;
        if position == end {
            return -cost;
        }
        if previous_direction != (0, 0) && distances.get((position, previous_direction)) < -cost {
            continue;
        }

        for (dx, dy) in DIRECTIONS {
            if previous_direction == (dx, dy) || previous_direction == (-dx, -dy) {
                continue;
            }

            let mut next_cost = -cost;

            for length in 1..=max_steps {
                let (nx, ny) = (x + dx * length, y + dy * length);
                if nx < 0 || nx >= board.width || ny < 0 || ny >= board.height {
                    break;
                }

                next_cost += board.get(nx, ny) as Cost;

                let key = ((nx, ny), (dx, dy));
                let shortest_distance = distances.get_mut(key);
                if length >= min_steps && next_cost < *shortest_distance {
                    *shortest_distance = next_cost;
                    queue.push((-next_cost, key));
                }
            }
        }
    }

    unreachable!()
}

fn solve_part01(input: &[u8]) -> i32 {
    let board = Board::parse(input);
    let start = (0, 0);
    let end = (board.width - 1, board.height - 1);
    shortest_path(&board, start, end, 1, 3)
}

fn solve_part02(input: &[u8]) -> i32 {
    let board = Board::parse(input);
    let start = (0, 0);
    let end = (board.width - 1, board.height - 1);
    shortest_path(&board, start, end, 4, 10)
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
