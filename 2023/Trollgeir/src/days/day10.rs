use crate::{Solution, SolutionPair};
use std::io::BufRead;

type Coord = (i32, i32);

trait Step {
    fn step(&self, direction: Direction) -> Coord;
}

impl Step for Coord {
    fn step(&self, direction: Direction) -> Coord {
        let coord_delta = direction.coord_delta();
        (self.0 + coord_delta.0, self.1 + coord_delta.1)
    }
}
trait PipeExit {
    fn next_direction(&self, direction: Direction) -> Option<Direction>;
}

impl PipeExit for u8 {
    fn next_direction(&self, direction: Direction) -> Option<Direction> {
        type D = Direction;
        match (self, direction) {
            (b'F', D::West) => Some(D::South),
            (b'F', D::North) => Some(D::East),
            (b'J', D::South) => Some(D::West),
            (b'J', D::East) => Some(D::North),
            (b'L', D::South) => Some(D::East),
            (b'L', D::West) => Some(D::North),
            (b'7', D::East) => Some(D::South),
            (b'7', D::North) => Some(D::West),
            (b'|', D::South) | (b'|', D::North) => Some(direction),
            (b'-', D::East) | (b'-', D::West) => Some(direction),
            _ => None,
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn coord_delta(self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}
struct PipeMap {
    cursor: Coord,
    dim: i32,
    matrix: &'static [u8],
}

impl PipeMap {
    fn new(matrix: &'static [u8]) -> PipeMap {
        let mut cursor = None;
        for (row, line) in matrix.lines().enumerate() {
            for (column, c) in line.unwrap().chars().enumerate() {
                if c == 'S' {
                    cursor = Some((row as i32, column as i32))
                }
            }
        }
        PipeMap {
            cursor: cursor.expect("Map has starting position"),
            dim: matrix.lines().next().unwrap().unwrap().len() as i32,
            matrix,
        }
    }

    #[inline]
    fn get(&self, coord: Coord) -> u8 {
        let index = coord.0 * self.dim + coord.1 + coord.0;
        self.matrix[index as usize]
    }
    #[inline]
    fn cursor_index(&self) -> usize {
        (self.cursor.0 * self.dim + self.cursor.1 + self.cursor.0) as usize
    }

    fn find_first_feasible_direction(&mut self) -> Option<Direction> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .iter()
        .find_map(|&direction| {
            self.get(self.cursor.step(direction))
                .next_direction(direction)
                .map(|_| direction)
        })
    }

    fn flow_step(&mut self, current_direction: Direction) -> Option<Direction> {
        let pipe_exit_direction = self.get(self.cursor).next_direction(current_direction)?;
        self.cursor = self.cursor.step(pipe_exit_direction);
        Some(pipe_exit_direction)
    }

    fn construct_loop(&mut self) -> Option<Vec<(usize, u8)>> {
        let mut pipe_loop_positions: Vec<(usize, u8)> = vec![];
        let mut flow_direction = self.find_first_feasible_direction()?;
        self.cursor = self.cursor.step(flow_direction);
        pipe_loop_positions.push((self.cursor_index(), self.get(self.cursor)));

        while let Some(pipe_exit_direction) = self.flow_step(flow_direction) {
            flow_direction = pipe_exit_direction;
            pipe_loop_positions.push((self.cursor_index(), self.get(self.cursor)));
        }
        pipe_loop_positions.sort_by(|(i, _), (j, _)| i.cmp(j));
        Some(pipe_loop_positions)
    }
}

pub fn solve() -> SolutionPair {
    // Part 1
    let map_data = include_bytes!("../../input/input.txt");
    let mut map = PipeMap::new(map_data);
    let pipe_loop_positions = map.construct_loop().unwrap();

    let half_length = pipe_loop_positions.len() / 2;

    // Part 2
    // Note: the 'S' pipe _might_ need to be resolved based on your input data

    let mut inner_loop_counts = 0;
    let mut inside = false;
    let mut current_pipe_origin = Direction::North;
    let mut last_pipe_index = 0;

    for (i, pipe) in pipe_loop_positions {
        let gap = i - last_pipe_index;
        if inside {
            inner_loop_counts += gap - 1;
        }

        match (pipe, current_pipe_origin) {
            (b'|', _) => inside = !inside,
            (b'L', _) => current_pipe_origin = Direction::North,
            (b'F', _) => current_pipe_origin = Direction::South,
            (b'J', Direction::South) => inside = !inside,
            (b'7', Direction::North) => inside = !inside,
            _ => {}
        }

        last_pipe_index = i;
    }

    (
        Solution::U32(half_length as u32),
        Solution::U32(inner_loop_counts as u32),
    )
}
