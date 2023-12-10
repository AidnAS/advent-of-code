use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[inline]
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    fn rev(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
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

const TILE_CONNECTIONS: [&[Direction]; 8] = [
    // S
    &[UP, RIGHT, DOWN, LEFT],
    // |
    &[UP, DOWN],
    // -
    &[LEFT, RIGHT],
    // L
    &[UP, RIGHT],
    // J
    &[LEFT, UP],
    // 7
    &[LEFT, DOWN],
    // F
    &[DOWN, RIGHT],
    // .
    &[],
];

#[inline]
fn clockwise_turn(incoming: &Direction, outgoing: &Direction) -> i8 {
    match (*incoming, *outgoing) {
        (DOWN, RIGHT) => 1,
        (LEFT, DOWN) => 1,
        (UP, LEFT) => 1,
        (RIGHT, UP) => 1,
        (DOWN, LEFT) => -1,
        (RIGHT, DOWN) => -1,
        (UP, RIGHT) => -1,
        (LEFT, UP) => -1,
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    value: u8,
}

impl Tile {
    #[inline]
    fn index(&self) -> usize {
        match self.value {
            b'S' => 0,
            b'|' => 1,
            b'-' => 2,
            b'L' => 3,
            b'J' => 4,
            b'7' => 5,
            b'F' => 6,
            b'.' => 7,
            _ => panic!("Invalid tile: {}", self.value),
        }
    }

    #[inline]
    fn is_pipe(&self) -> bool {
        self.index() < 7
    }

    #[inline]
    fn outgoing_directions(&self, incoming: Option<Direction>) -> impl Iterator<Item = &Direction> {
        TILE_CONNECTIONS[self.index()]
            .iter()
            .filter(move |&outgoing_direction| match incoming {
                Some(incoming_direction) => incoming_direction != *outgoing_direction,
                None => true,
            })
    }

    #[inline]
    fn valid_incoming(&self, incoming: Direction) -> bool {
        TILE_CONNECTIONS[self.index()].contains(&incoming)
    }
}

impl From<u8> for Tile {
    fn from(byte: u8) -> Self {
        Self { value: byte }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value as char)
    }
}

struct Tiles {
    vec: Vec<Tile>,
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
                line.iter().map(|&b| Tile::from(b))
            })
            .collect();
        let height = input.len() / width;
        Self { vec, width, height }
    }
}

#[allow(dead_code)]
impl Tiles {
    #[inline]
    fn get(&self, point: Point) -> Option<&Tile> {
        if point.x < 0
            || point.x as usize >= self.width
            || point.y < 0
            || point.y as usize >= self.height
        {
            return None;
        }
        self.vec
            .get((point.y * self.width as i32 + point.x) as usize)
    }

    #[inline]
    fn set(&mut self, point: Point, value: Tile) {
        if point.x < 0
            || point.x as usize >= self.width
            || point.y < 0
            || point.y as usize >= self.height
        {
            return;
        }
        self.vec[point.y as usize * self.width + point.x as usize] = value;
    }

    #[inline]
    fn find(&self, tile: Tile) -> Option<Point> {
        self.vec
            .iter()
            .position(|&t| t.value == tile.value)
            .map(|index| {
                Point::new(
                    index as i32 % self.width as i32,
                    index as i32 / self.width as i32,
                )
            })
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.vec[y * self.width + x]);
            }
            println!();
        }
    }
}

fn solve_part01(input: &[u8]) -> u32 {
    let tiles = Tiles::from(input);
    let start_tile = Tile::from(b'S');
    let start_position = tiles.find(start_tile).unwrap();

    // Since the path is a pipe, i.e, a single cycle, we can just iterate around the pipe once
    // we find the start position and a valid outgoing direction.
    // Using a DFS until we're back at the start position.
    let mut queue: Vec<(Point, Option<Direction>)> = Vec::with_capacity(tiles.vec.len());
    let mut path = Vec::with_capacity(tiles.vec.len());
    queue.push((start_position, None));
    while let Some((position, incoming_direction)) = queue.pop() {
        let tile = tiles.get(position).unwrap();
        for outgoing_direction in tile.outgoing_directions(incoming_direction) {
            let next_position = position + *outgoing_direction;
            let next_incoming_direction = outgoing_direction.rev();
            if let Some(next_tile) = tiles.get(next_position) {
                if next_tile.is_pipe() && next_tile.valid_incoming(next_incoming_direction) {
                    queue.push((next_position, Some(next_incoming_direction)));
                    path.push(next_position);
                    if next_tile.value == b'S' {
                        queue.clear();
                    }
                    break;
                }
            }
        }
    }

    (path.len() / 2) as u32
}

fn solve_part02(input: &[u8]) -> u32 {
    let tiles = Tiles::from(input);
    let start_tile = Tile::from(b'S');
    let start_position = tiles.find(start_tile).unwrap();

    // Since the path is a pipe, i.e, a single cycle, we can just iterate around the pipe once
    // we find the start position and a valid outgoing direction.
    // Using a DFS until we're back at the start position.
    let mut queue: Vec<(Point, Option<Direction>)> = Vec::with_capacity(tiles.vec.len());
    let mut path = Vec::with_capacity(tiles.vec.len());
    path.push(start_position);
    queue.push((start_position, None));
    let mut net_clockwise_turns = 0;

    while let Some((position, incoming_direction)) = queue.pop() {
        let tile = tiles.get(position).unwrap();
        for outgoing_direction in tile.outgoing_directions(incoming_direction) {
            let next_position = position + *outgoing_direction;
            let next_incoming_direction = outgoing_direction.rev();
            if let Some(next_tile) = tiles.get(next_position) {
                if next_tile.is_pipe() && next_tile.valid_incoming(next_incoming_direction) {
                    queue.push((next_position, Some(next_incoming_direction)));
                    path.push(next_position);
                    if next_tile.value == b'S' {
                        queue.clear();
                    }
                    if let Some(incoming) = incoming_direction {
                        net_clockwise_turns += clockwise_turn(&incoming, outgoing_direction);
                    };
                    break;
                }
            }
        }
    }

    // Add the second to last position to the start of the path so we can iterate over windows
    path.insert(0, *path.iter().rev().nth(1).unwrap());

    // Have to iterate clockwise to find the inner points
    if net_clockwise_turns < 0 {
        path.reverse();
    }

    let path_points = path.iter().collect::<HashSet<_>>();
    let mut inner_starting_points = VecDeque::new();
    for window in path.windows(3) {
        let (a, b, c) = (window[0], window[1], window[2]);
        let incoming_direction: Direction = b - a;
        let outgoing_direction: Direction = c - b;

        match (incoming_direction, outgoing_direction) {
            (UP, UP) => inner_starting_points.push_back(b + RIGHT),
            (RIGHT, RIGHT) => inner_starting_points.push_back(b + DOWN),
            (DOWN, DOWN) => inner_starting_points.push_back(b + LEFT),
            (LEFT, LEFT) => inner_starting_points.push_back(b + UP),
            (UP, LEFT) => inner_starting_points.extend([b + RIGHT, b + UP]),
            (LEFT, DOWN) => inner_starting_points.extend([b + UP, b + LEFT]),
            (DOWN, RIGHT) => inner_starting_points.extend([b + LEFT, b + DOWN]),
            (RIGHT, UP) => inner_starting_points.extend([b + DOWN, b + RIGHT]),
            _ => {}
        }
    }

    let mut queue = inner_starting_points;
    queue.retain(|point| tiles.get(*point).is_some() && !path_points.contains(point));

    let mut inner_points = HashSet::new();
    // BFS from all the inner starting points
    while let Some(point) = queue.pop_front() {
        if !inner_points.contains(&point) {
            inner_points.insert(point);
        }
        for direction in &[UP, RIGHT, DOWN, LEFT] {
            let next_point = point + *direction;
            if tiles.get(next_point).is_some()
                && !inner_points.contains(&next_point)
                && !path_points.contains(&next_point)
            {
                queue.push_back(next_point);
            }
        }
    }

    inner_points.len() as u32
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
