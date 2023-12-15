use std::ops::{Add, Sub};

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

type Direction = Point;

const UP: Direction = Direction { x: 0, y: -1 };
const RIGHT: Direction = Direction { x: 1, y: 0 };
const DOWN: Direction = Direction { x: 0, y: 1 };
const LEFT: Direction = Direction { x: -1, y: 0 };

struct Tiles {
    vec: Vec<u8>,
    width: usize,
    height: usize,
}

impl From<&[u8]> for Tiles {
    fn from(input: &[u8]) -> Self {
        let mut width = 0;
        let vec = input
            .split(|&b| b == b'\n')
            .flat_map(|line| {
                width = line.len();
                line.iter().copied()
            })
            .collect();
        let height = input.len() / width;
        Self { vec, width, height }
    }
}

#[allow(dead_code)]
impl Tiles {
    #[inline]
    fn get(&self, point: Point) -> u8 {
        self.vec[point.y as usize * self.width + point.x as usize]
    }

    #[inline]
    fn find(&self, tile: u8) -> Option<Point> {
        self.vec.iter().position(|&t| t == tile).map(|index| Point {
            x: index as i32 % self.width as i32,
            y: index as i32 / self.width as i32,
        })
    }

    #[inline]
    fn next_position(&self, position: Point, previous_position: Point) -> Point {
        let delta = position - previous_position;
        let tile = self.get(position);
        match (delta, tile) {
            (UP, b'|') => position + UP,
            (DOWN, b'|') => position + DOWN,
            (LEFT, b'-') => position + LEFT,
            (RIGHT, b'-') => position + RIGHT,
            (DOWN, b'L') => position + RIGHT,
            (LEFT, b'L') => position + UP,
            (DOWN, b'J') => position + LEFT,
            (RIGHT, b'J') => position + UP,
            (LEFT, b'F') => position + DOWN,
            (UP, b'F') => position + RIGHT,
            (RIGHT, b'7') => position + DOWN,
            (UP, b'7') => position + LEFT,
            (_, b'S') => {
                let dir = [UP, DOWN, RIGHT, LEFT]
                    .iter()
                    .find(|&dir| {
                        let next_position = position + *dir;
                        if next_position.x >= 0
                            && next_position.x < self.width as i32
                            && next_position.y >= 0
                            && next_position.y < self.height as i32
                        {
                            let next_tile = self.get(next_position);
                            matches!(
                                (*dir, next_tile),
                                (UP, b'|' | b'F' | b'7')
                                    | (RIGHT, b'-' | b'7' | b'J')
                                    | (DOWN, b'|' | b'L' | b'J')
                                    | (LEFT, b'-' | b'L' | b'F')
                            )
                        } else {
                            false
                        }
                    })
                    .unwrap();
                position + *dir
            }
            (_, _) => unreachable!(),
        }
    }
}

fn find_path(tiles: &Tiles) -> Vec<Point> {
    let mut path = Vec::with_capacity(tiles.vec.len());
    let mut current_position = tiles.find(b'S').unwrap();
    let mut previous_position = current_position;

    loop {
        path.push(current_position);
        let next_position = tiles.next_position(current_position, previous_position);
        previous_position = current_position;
        current_position = next_position;

        if tiles.get(current_position) == b'S' {
            path.push(current_position);
            break;
        }
    }

    path
}

fn solve_part01(input: &[u8]) -> u32 {
    let tiles = Tiles::from(input);
    let path = find_path(&tiles);
    (path.len() / 2) as u32
}

fn solve_part02(input: &[u8]) -> i32 {
    let tiles = Tiles::from(input);
    let path = find_path(&tiles);

    let circumference = path.len() as i32 - 1;
    let area: i32 = path
        .windows(2)
        .map(|w| {
            let (a, b) = (w[0], w[1]);
            a.x * b.y - a.y * b.x
        })
        .sum();

    (area.abs() - circumference) / 2 + 1
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
    fn day10_part01_1() {
        let input = include_bytes!("input_test01.txt");
        assert_eq!(solve_part01(input), 4);
    }

    #[test]
    fn day10_part01_2() {
        let input = include_bytes!("input_test02.txt");
        assert_eq!(solve_part01(input), 8);
    }

    #[test]
    fn day10_part02_1() {
        let input = include_bytes!("input_test03.txt");
        assert_eq!(solve_part02(input), 4);
    }

    #[test]
    fn day10_part02_2() {
        let input = include_bytes!("input_test04.txt");
        assert_eq!(solve_part02(input), 8);
    }

    #[test]
    fn day10_part02_3() {
        let input = include_bytes!("input_test05.txt");
        assert_eq!(solve_part02(input), 10);
    }
}
