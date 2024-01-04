use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

fn settle_bricks(bricks_input: &[Brick]) -> Vec<Brick> {
    let mut bricks = bricks_input.to_owned();
    let mut settled = false;
    while !settled {
        let mut new_bricks = Vec::new();
        settled = true;
        let mut points = HashSet::new();
        for brick in &bricks {
            for x in brick.start.0..=brick.end.0 {
                for y in brick.start.1..=brick.end.1 {
                    for z in brick.start.2..=brick.end.2 {
                        points.insert((x, y, z));
                    }
                }
            }
        }
        for brick in &bricks {
            let mut brick_settled = false;
            for x in brick.start.0..=brick.end.0 {
                for y in brick.start.1..=brick.end.1 {
                    if brick.start.2 - 1 == 0 || points.contains(&(x, y, brick.start.2 - 1)) {
                        brick_settled = true;
                    }
                }
            }
            let mut new_brick = *brick;
            if !brick_settled {
                new_brick.start.2 -= 1;
                new_brick.end.2 -= 1;
                settled = false;
            }
            new_bricks.push(new_brick);
        }
        bricks = new_bricks
    }
    bricks
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let mut start = start.split(',').map(|s| s.parse::<usize>().unwrap());
            let mut end = end.split(',').map(|s| s.parse::<usize>().unwrap());
            Brick {
                start: (
                    start.next().unwrap(),
                    start.next().unwrap(),
                    start.next().unwrap(),
                ),
                end: (
                    end.next().unwrap(),
                    end.next().unwrap(),
                    end.next().unwrap(),
                ),
            }
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    let mut bricks = parse(input);
    bricks = settle_bricks(&bricks);
    bricks.iter().enumerate().fold(0, |mut acc, (i, _)| {
        let mut brick_disintegrated = bricks.clone();
        brick_disintegrated.remove(i);
        let settled_bricks = settle_bricks(&brick_disintegrated);
        if settled_bricks == brick_disintegrated {
            acc += 1;
        }
        acc
    })
}

fn part2(input: &str) -> usize {
    let mut bricks = parse(input);
    bricks = settle_bricks(&bricks);
    bricks.iter().enumerate().fold(0, |mut acc, (i, _)| {
        let mut brick_disintegrated = bricks.clone();
        brick_disintegrated.remove(i);
        let mut settled_bricks = settle_bricks(&brick_disintegrated);
        if settled_bricks != brick_disintegrated {
            let mut settled = false;
            while !settled {
                let new_bricks = settle_bricks(&settled_bricks);
                if new_bricks == settled_bricks {
                    settled = true;
                }
                settled_bricks = new_bricks;
            }
            acc += settled_bricks
                .iter()
                .enumerate()
                .filter(|&(i, &brick)| brick != brick_disintegrated[i])
                .count();
        }
        acc
    })
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
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 7);
    }
}
