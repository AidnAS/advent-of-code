use std::collections::HashSet;

fn bfs(map: &[Vec<char>], start: (isize, isize), max_steps: usize) -> usize {
    let mut current = HashSet::new();
    let mut next = HashSet::new();
    current.insert(start);
    let max_y = map.len() as isize;
    let max_x = map[0].len() as isize;
    for _ in 0..max_steps {
        current.iter().for_each(|(current_y, current_x)| {
            [(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .for_each(|(y, x)| {
                    let new_y = *current_y + y;
                    let new_x = *current_x + x;
                    if map[new_y.rem_euclid(max_y) as usize][new_x.rem_euclid(max_x) as usize]
                        != '#'
                    {
                        next.insert((new_y, new_x));
                    }
                });
        });
        (current, next) = (next, current);
        next.clear();
    }
    current.len()
}

fn part1(input: &str, max_steps: usize) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&tile| tile == 'S')
                .map(|x| (y as isize, x as isize))
        })
        .unwrap();

    bfs(&map, start, max_steps)
}

fn part2(input: &str, max_steps: usize) -> isize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&tile| tile == 'S')
                .map(|x| (y as isize, x as isize))
        })
        .unwrap();

    // https://www.radfordmathematics.com/algebra/sequences-series/difference-method-sequences/quadratic-sequences.html
    let steps_to_edge = max_steps % map.len();
    let u1 = bfs(&map, start, steps_to_edge) as isize;
    let u2 = bfs(&map, start, steps_to_edge + map.len()) as isize;
    let u3 = bfs(&map, start, steps_to_edge + map.len() * 2) as isize;
    let second_difference = (u3 - u2) - (u2 - u1);
    let a = second_difference / 2;
    let b = (u2 - u1) - 3 * a;
    let c = u1 - a - b;
    let n = (max_steps as f64 / map.len() as f64).ceil() as isize;
    a * n * n + b * n + c
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input, 64));
    println!("part2: {}", part2(input, 26_501_365));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input, 6), 16);
        assert_eq!(part1(input, 10), 50);
        assert_eq!(part1(input, 50), 1594);
        assert_eq!(part1(input, 100), 6536);
    }
}
