use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    cost: usize,
    y: usize,
    x: usize,
    direction: Direction,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbours(
    grid: &Vec<Vec<usize>>,
    current: State,
    min_steps: usize,
    max_steps: usize,
) -> Vec<State> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut neighbours = Vec::new();
    for dir in [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ] {
        if current.steps < min_steps && dir != current.direction {
            continue;
        }
        if current.direction == dir && current.steps == max_steps {
            continue;
        }

        let next = match dir {
            Direction::North if current.direction != Direction::South && current.y > 0 => {
                Some((current.y - 1, current.x))
            }
            Direction::South if current.direction != Direction::North && current.y < (rows - 1) => {
                Some((current.y + 1, current.x))
            }
            Direction::West if current.direction != Direction::East && current.x > 0 => {
                Some((current.y, current.x - 1))
            }
            Direction::East if current.direction != Direction::West && current.x < (cols - 1) => {
                Some((current.y, current.x + 1))
            }
            _ => None,
        };

        if let Some((y, x)) = next {
            let cost = current.cost + grid[y][x];

            let mut steps = 1;
            if current.direction == dir {
                steps = current.steps + 1;
            }
            neighbours.push(State {
                cost,
                y,
                x,
                direction: dir,
                steps,
            })
        }
    }
    neighbours
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn shortest_path(input: &str, min_steps: usize, max_steps: usize) -> usize {
    let grid = parse(input);
    let goal = (grid.len() - 1, grid[0].len() - 1);

    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();

    let right = State {
        cost: grid[0][1],
        y: 0,
        x: 1,
        direction: Direction::East,
        steps: 1,
    };
    queue.push(right);

    let down = State {
        cost: grid[1][0],
        y: 1,
        x: 0,
        direction: Direction::South,
        steps: 1,
    };
    queue.push(down);

    while let Some(tile) = queue.pop() {
        if tile.y == goal.0 && tile.x == goal.1 && tile.steps >= min_steps {
            return tile.cost;
        }
        if tile.cost
            > *distances
                .get(&(tile.y, tile.x, tile.direction, tile.steps))
                .unwrap_or(&usize::MAX)
        {
            continue;
        }
        for neighbour in get_neighbours(&grid, tile, min_steps, max_steps) {
            if neighbour.cost
                < *distances
                    .get(&(
                        neighbour.y,
                        neighbour.x,
                        neighbour.direction,
                        neighbour.steps,
                    ))
                    .unwrap_or(&usize::MAX)
            {
                queue.push(neighbour);
                distances.insert(
                    (
                        neighbour.y,
                        neighbour.x,
                        neighbour.direction,
                        neighbour.steps,
                    ),
                    neighbour.cost,
                );
            }
        }
    }
    unreachable!()
}

fn part1(input: &str) -> usize {
    shortest_path(input, 1, 3)
}

fn part2(input: &str) -> usize {
    shortest_path(input, 4, 10)
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {:?}", part1(input));
    println!("part2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input1.txt");
        assert_eq!(part1(input), 102);
    }

    #[test]
    fn test_part2() {
        let input1 = include_str!("test_input1.txt");
        assert_eq!(part2(input1), 94);
        let input2 = include_str!("test_input2.txt");
        assert_eq!(part2(input2), 71);
    }
}
