use std::collections::HashSet;

use crate::solution::Solution;

type IndexType = i16;

type Position = (IndexType, IndexType);
type Direction = (IndexType, IndexType);
type Ray = (Position, Direction);

const UP: Direction = (0, -1);
const DOWN: Direction = (0, 1);
const RIGHT: Direction = (1, 0);
const LEFT: Direction = (-1, 0);

#[derive(Debug, Clone)]
struct Board<'a> {
    bytes: &'a [u8],
    width: IndexType,
    height: IndexType,

    visited: HashSet<Position>,
    rays: HashSet<Ray>,
    queue: Vec<Ray>,
}

impl<'a> Board<'a> {
    pub fn parse(input: &'a [u8]) -> Self {
        let width = (input.iter().position(|&b| b == b'\n').unwrap()) as IndexType;
        let height = (input.len() / (width) as usize) as IndexType;

        Self {
            bytes: input,
            width,
            height,
            rays: HashSet::new(),
            visited: HashSet::new(),
            queue: Vec::new(),
        }
    }

    pub fn beam(&mut self, start_ray: Ray) -> u32 {
        self.rays.clear();
        self.visited.clear();
        self.queue.clear();
        self.queue.push(start_ray);

        while let Some(ray) = self.queue.pop() {
            let (position, direction) = ray;
            let ((x, y), (dx, dy)) = (position, direction);
            let (nx, ny) = (x + dx, y + dy);
            if self.rays.contains(&ray) {
                continue;
            }
            self.rays.insert(ray);
            self.visited.insert(position);

            if nx < 0 || nx >= self.width || ny < 0 || ny >= self.height {
                continue;
            }

            match (self.bytes[(ny * (self.width + 1) + nx) as usize], (dx, dy)) {
                (b'.', _) => {
                    self.queue.push(((nx, ny), (dx, dy)));
                }
                (b'|', UP | DOWN) | (b'-', LEFT | RIGHT) => {
                    self.queue.push(((nx, ny), direction));
                }
                (b'|', LEFT | RIGHT) => {
                    self.queue.push(((nx, ny), UP));
                    self.queue.push(((nx, ny), DOWN));
                }
                (b'-', UP | DOWN) => {
                    self.queue.push(((nx, ny), LEFT));
                    self.queue.push(((nx, ny), RIGHT));
                }
                (b'/', RIGHT) | (b'\\', LEFT) => {
                    self.queue.push(((nx, ny), UP));
                }
                (b'/', LEFT) | (b'\\', RIGHT) => {
                    self.queue.push(((nx, ny), DOWN));
                }
                (b'/', UP) | (b'\\', DOWN) => {
                    self.queue.push(((nx, ny), RIGHT));
                }
                (b'/', DOWN) | (b'\\', UP) => {
                    self.queue.push(((nx, ny), LEFT));
                }
                _ => unreachable!(),
            }
        }
        self.visited.len() as u32
    }
}
fn solve_part01(input: &[u8]) -> u32 {
    let mut board = Board::parse(input);
    board.beam(((0, 0), RIGHT))
}

fn solve_part02(input: &[u8]) -> u32 {
    let mut board = Board::parse(input);
    let width = board.width;
    let height = board.height;
    (0..height)
        .flat_map(|y| [((0, y), RIGHT), ((width - 1, y), LEFT)])
        .chain((0..width).flat_map(|x| [((x, 0), DOWN), ((x, height - 1), UP)]))
        .map(|ray| board.beam(ray))
        .max()
        .unwrap()
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
    fn day16_part01() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part01(input), 46);
    }

    #[test]
    fn day16_part02() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part02(input), 51);
    }
}
