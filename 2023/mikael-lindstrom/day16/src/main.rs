use std::collections::HashSet;

fn get_energized(map: &Vec<Vec<char>>, y: usize, x: usize, dir: usize) -> usize {
    let mut queue = vec![(y, x, dir)];
    let mut energized = HashSet::new();
    while let Some((y, x, dir)) = queue.pop() {
        if energized.get(&(y, x, dir)).is_some() {
            continue;
        }
        energized.insert((y, x, dir));
        match dir {
            0 => match map[y][x] {
                '\\' if x > 0 => {
                    queue.push((y, x - 1, 3));
                }
                '/' if x + 1 < map[0].len() => {
                    queue.push((y, x + 1, 1));
                }
                '-' => {
                    if x > 0 {
                        queue.push((y, x - 1, 3));
                    }
                    if x + 1 < map[0].len() {
                        queue.push((y, x + 1, 1));
                    }
                }
                '.' | '|' if y > 0 => {
                    queue.push((y - 1, x, 0));
                }
                _ => (),
            },
            1 => match map[y][x] {
                '\\' if y + 1 < map.len() => {
                    queue.push((y + 1, x, 2));
                }
                '/' if y > 0 => {
                    queue.push((y - 1, x, 0));
                }
                '|' => {
                    if y > 0 {
                        queue.push((y - 1, x, 0));
                    }
                    if y + 1 < map.len() {
                        queue.push((y + 1, x, 2));
                    }
                }
                '.' | '-' if x + 1 < map[0].len() => {
                    queue.push((y, x + 1, 1));
                }
                _ => (),
            },
            2 => match map[y][x] {
                '\\' if x + 1 < map[0].len() => {
                    queue.push((y, x + 1, 1));
                }
                '/' if x > 0 => {
                    queue.push((y, x - 1, 3));
                }
                '-' => {
                    if x > 0 {
                        queue.push((y, x - 1, 3));
                    }
                    if x + 1 < map[0].len() {
                        queue.push((y, x + 1, 1));
                    }
                }
                '.' | '|' if y + 1 < map.len() => {
                    queue.push((y + 1, x, 2));
                }
                _ => (),
            },
            3 => match map[y][x] {
                '\\' if y > 0 => {
                    queue.push((y - 1, x, 0));
                }
                '/' if y + 1 < map.len() => {
                    queue.push((y + 1, x, 2));
                }
                '|' => {
                    if y > 0 {
                        queue.push((y - 1, x, 0));
                    }
                    if y + 1 < map.len() {
                        queue.push((y + 1, x, 2));
                    }
                }
                '.' | '-' if x > 0 => {
                    queue.push((y, x - 1, 3));
                }
                _ => (),
            },
            _ => (),
        }
    }
    energized
        .iter()
        .fold(HashSet::new(), |mut acc, (y, x, _)| {
            acc.insert((y, x));
            acc
        })
        .len()
}

fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    get_energized(&map, 0, 0, 1)
}

fn part2(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut positions = Vec::new();
    for y in 0..map.len() {
        positions.push((y, 0, 1));
        positions.push((y, map[0].len() - 1, 3));
    }
    for x in 0..map[0].len() {
        positions.push((0, x, 2));
        positions.push((map.len() - 1, x, 0));
    }
    positions
        .iter()
        .map(|&(y, x, dir)| get_energized(&map, y, x, dir))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input), 46);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 51);
    }
}
