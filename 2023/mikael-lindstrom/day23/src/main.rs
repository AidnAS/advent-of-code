#![allow(clippy::type_complexity)]
use std::collections::{HashMap, HashSet};

fn get_start_end(input: &str) -> ((isize, isize), (isize, isize)) {
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();
    ((0, 1), (max_y as isize - 1, max_x as isize - 2))
}

fn parse(input: &str) -> HashMap<(isize, isize), HashMap<(isize, isize), isize>> {
    let nodes = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (x, c)| {
                    if c == '#' {
                        return acc;
                    }
                    let mut neighbours = Vec::<(isize, isize)>::new();
                    match c {
                        '^' => {
                            neighbours.push((-1, 0));
                        }
                        '>' => {
                            neighbours.push((0, 1));
                        }
                        'v' => {
                            neighbours.push((1, 0));
                        }
                        '<' => {
                            neighbours.push((0, -1));
                        }
                        '.' => {
                            neighbours.push((-1, 0));
                            neighbours.push((0, 1));
                            neighbours.push((1, 0));
                            neighbours.push((0, -1));
                        }
                        _ => unreachable!(),
                    }
                    acc.insert((y as isize, x as isize), neighbours);
                    acc
                })
        })
        .collect::<HashMap<_, _>>();
    nodes
        .iter()
        .map(|((y, x), neighbours)| {
            let neighbours = neighbours
                .iter()
                .filter_map(|(y1, x1)| {
                    if nodes.contains_key(&(y + y1, x + x1)) {
                        return Some(((y + y1, x + x1), 1));
                    }
                    None
                })
                .collect::<HashMap<_, _>>();
            ((*y, *x), neighbours)
        })
        .collect::<HashMap<_, _>>()
}

fn longest_distance(
    position: (isize, isize),
    end: (isize, isize),
    distance: isize,
    seen: &mut HashSet<(isize, isize)>,
    nodes: &HashMap<(isize, isize), HashMap<(isize, isize), isize>>,
) -> isize {
    if position == end {
        return distance;
    }
    if !seen.insert(position) {
        return 0;
    }

    let max = nodes
        .get(&(position))
        .unwrap()
        .iter()
        .map(|(&(y, x), dist)| longest_distance((y, x), end, distance + dist, seen, nodes))
        .max()
        .unwrap();

    seen.remove(&position);
    max
}

fn compress_graph(nodes: &mut HashMap<(isize, isize), HashMap<(isize, isize), isize>>) {
    let nodes_to_compress = nodes
        .clone()
        .into_iter()
        .filter(|(_, neighbours)| neighbours.len() == 2)
        .collect::<Vec<_>>();
    for (position, _) in nodes_to_compress {
        let neighbours = nodes.remove(&position).unwrap();
        let mut neighbours_iter = neighbours.iter();
        let (position1, distance1) = neighbours_iter.next().unwrap();
        let (position2, distance2) = neighbours_iter.next().unwrap();
        let node1 = nodes.get_mut(position1).unwrap();
        node1.remove(&position);
        node1.insert(*position2, distance1 + distance2);
        let node2 = nodes.get_mut(position2).unwrap();
        node2.remove(&position);
        node2.insert(*position1, distance1 + distance2);
    }
}

fn part1(input: &str) -> usize {
    let (start, end) = get_start_end(input);
    let nodes = parse(input);
    longest_distance(start, end, 0, &mut HashSet::new(), &nodes) as usize
}

fn part2(input: &str) -> usize {
    let (start, end) = get_start_end(input);
    let mut nodes = parse(&input.replace(['^', '>', 'v', '<'], "."));
    compress_graph(&mut nodes);
    longest_distance(start, end, 0, &mut HashSet::new(), &nodes) as usize
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
        assert_eq!(part1(input), 94);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 154);
    }
}
