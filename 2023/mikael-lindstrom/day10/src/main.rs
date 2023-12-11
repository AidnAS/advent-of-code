use std::collections::{HashSet, VecDeque};

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let tiles = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start_tile = tiles
        .iter()
        .enumerate()
        .find_map(|(y, tile_row)| {
            tile_row
                .iter()
                .position(|&tile| tile == 'S')
                .map(|x| (y, x))
        })
        .unwrap();
    (tiles, start_tile)
}

fn get_neighbours(tiles: &[Vec<char>], start_tile: (usize, usize)) -> [bool; 4] {
    let max_y = tiles.len() as i32;
    let max_x = tiles[0].len() as i32;

    // up, right, down, left
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|(y, x)| {
            let new_y = start_tile.0 as i32 + y;
            let new_x = start_tile.1 as i32 + x;
            if new_y >= 0 && new_y < max_y && new_x >= 0 && new_x < max_x {
                return match tiles[new_y as usize][new_x as usize] {
                    '|' | '7' | 'F' if *y < 0 => true,
                    '-' | '7' | 'J' if *x > 0 => true,
                    '|' | 'L' | 'J' if *y > 0 => true,
                    '-' | 'L' | 'F' if *x < 0 => true,
                    _ => false,
                };
            }
            false
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn get_loop_nodes(tiles: &[Vec<char>], start_tile: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(start_tile);
    while let Some(tile) = queue.pop_front() {
        if !seen.contains(&tile) {
            match tiles.get(tile.0).unwrap().get(tile.1).unwrap() {
                '|' => {
                    queue.push_back((tile.0 - 1, tile.1));
                    queue.push_back((tile.0 + 1, tile.1));
                }
                '-' => {
                    queue.push_back((tile.0, tile.1 - 1));
                    queue.push_back((tile.0, tile.1 + 1));
                }
                'L' => {
                    queue.push_back((tile.0 - 1, tile.1));
                    queue.push_back((tile.0, tile.1 + 1));
                }
                'J' => {
                    queue.push_back((tile.0 - 1, tile.1));
                    queue.push_back((tile.0, tile.1 - 1));
                }
                '7' => {
                    queue.push_back((tile.0 + 1, tile.1));
                    queue.push_back((tile.0, tile.1 - 1));
                }
                'F' => {
                    queue.push_back((tile.0 + 1, tile.1));
                    queue.push_back((tile.0, tile.1 + 1));
                }
                'S' => {
                    let [up, right, down, left] = get_neighbours(tiles, start_tile);
                    if up {
                        queue.push_back((tile.0 - 1, tile.1));
                    }
                    if right {
                        queue.push_back((tile.0, tile.1 + 1));
                    }
                    if down {
                        queue.push_back((tile.0 + 1, tile.1));
                    }
                    if left {
                        queue.push_back((tile.0, tile.1 - 1));
                    }
                }
                _ => (),
            }
            seen.insert(tile);
        }
    }
    seen
}

fn part1(input: &str) -> u32 {
    let (tiles, start_tile) = parse(input);
    (get_loop_nodes(&tiles, start_tile).len() / 2) as u32
}

fn part2(input: &str) -> u32 {
    let (tiles, start_tile) = parse(input);

    let loop_nodes = get_loop_nodes(&tiles, start_tile);
    let neighbours = get_neighbours(&tiles, start_tile);
    let is_start_vaild = match neighbours {
        // "|", "F", "7"
        [true, false, true, false] | [false, true, true, false] | [false, false, true, true] => {
            true
        }
        [_, _, _, _] => false,
    };

    (0..tiles.len())
        .map(|y| {
            let mut inside = false;
            (0..tiles[y].len())
                .map(|x| {
                    let current = tiles[y][x];
                    match current {
                        'S' if is_start_vaild => {
                            inside = !inside;
                            0
                        }
                        '|' | '7' | 'F' if loop_nodes.contains(&(y, x)) => {
                            inside = !inside;
                            0
                        }
                        _ if inside && !loop_nodes.contains(&(y, x)) => 1,
                        _ => 0,
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
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
        let input_a = include_str!("test_input_1a.txt");
        assert_eq!(part1(input_a), 4);
        let input_b = include_str!("test_input_1b.txt");
        assert_eq!(part1(input_b), 8);
    }

    #[test]
    fn test_part2() {
        let input_a = include_str!("test_input_2a.txt");
        assert_eq!(part2(input_a), 4);
        let input_b = include_str!("test_input_2b.txt");
        assert_eq!(part2(input_b), 8);
        let input_c = include_str!("test_input_2c.txt");
        assert_eq!(part2(input_c), 10);
    }
}
