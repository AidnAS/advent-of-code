use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

use crate::solution::Solution;

use num::integer::lcm;

#[derive(Default, Debug, Clone, Copy, Eq)]
struct Node {
    bytes: [u8; 3],
    index: u16,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl Node {
    #[inline]
    fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.try_into().unwrap(),
            index: 0,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connection {
    from: Node,
    left: Node,
    right: Node,
}

fn solve_part01(input: &[u8]) -> u32 {
    let mut line_iter = input.split(|&b| b == b'\n');
    let directions = line_iter.next().unwrap();
    line_iter.next(); // Skip empty line

    let unindexed_connections = line_iter
        .map(|line| {
            let from = Node::new(&line[0..3]);
            let left = Node::new(&line[7..10]);
            let right = Node::new(&line[12..15]);
            Connection { from, left, right }
        })
        .collect::<Vec<Connection>>();

    // Map all nodes to an index so that the traversing is faster
    let mut nodes_map: HashMap<Node, u16> = HashMap::with_capacity(unindexed_connections.len());
    let mut connections = vec![Default::default(); unindexed_connections.len()];
    let mut current_node = Node::default();
    let mut next_index = 0;
    for mut connection in unindexed_connections {
        let from_index = *nodes_map.entry(connection.from).or_insert_with(|| {
            let index = next_index;
            next_index += 1;
            index
        });
        connection.from.index = from_index;
        let left_index = *nodes_map.entry(connection.left).or_insert_with(|| {
            let index = next_index;
            next_index += 1;
            index
        });
        connection.left.index = left_index;
        let right_index = *nodes_map.entry(connection.right).or_insert_with(|| {
            let index = next_index;
            next_index += 1;
            index
        });
        connection.right.index = right_index;

        if connection.from.bytes == [b'A', b'A', b'A'] {
            current_node = connection.from;
        }

        connections[from_index as usize] = connection;
    }

    let mut steps = 0;

    'outer: loop {
        for dir in directions {
            steps += 1;
            current_node = match dir {
                b'L' => connections[current_node.index as usize].left,
                b'R' => connections[current_node.index as usize].right,
                _ => unreachable!(),
            };

            if current_node.bytes == [b'Z', b'Z', b'Z'] {
                break 'outer;
            }
        }
    }

    steps
}

fn solve_part02(input: &[u8]) -> u64 {
    let mut line_iter = input.split(|&b| b == b'\n');
    let directions = line_iter.next().unwrap();
    line_iter.next(); // Skip empty line

    let unindexed_connections = line_iter
        .map(|line| {
            let from = Node::new(&line[0..3]);
            let left = Node::new(&line[7..10]);
            let right = Node::new(&line[12..15]);
            Connection { from, left, right }
        })
        .collect::<Vec<Connection>>();

    // Map all nodes to an index so that the traversing is faster
    let mut nodes_map: HashMap<Node, u16> = HashMap::with_capacity(unindexed_connections.len());
    let mut connections = vec![Default::default(); unindexed_connections.len()];
    let mut current_nodes: Vec<(Node, u64)> = Vec::with_capacity(10);
    let mut next_index = 0;
    for mut connection in unindexed_connections {
        let from_index = *nodes_map.entry(connection.from).or_insert_with(|| {
            let index = next_index;
            next_index += 1;
            index
        });
        connection.from.index = from_index;
        let left_index = *nodes_map.entry(connection.left).or_insert_with(|| {
            let index = next_index;
            next_index += 1;
            index
        });
        connection.left.index = left_index;
        let right_index = *nodes_map.entry(connection.right).or_insert_with(|| {
            let index = next_index;
            next_index += 1;
            index
        });
        connection.right.index = right_index;

        if connection.from.bytes[2] == b'A' {
            current_nodes.push((connection.from, 0));
        }

        connections[from_index as usize] = connection;
    }

    for (current_node, steps) in current_nodes.iter_mut() {
        'outer: loop {
            for dir in directions {
                *steps += 1;
                *current_node = match dir {
                    b'L' => connections[current_node.index as usize].left,
                    b'R' => connections[current_node.index as usize].right,
                    _ => unreachable!(),
                };
                if current_node.bytes[2] == b'Z' {
                    break 'outer;
                }
            }
        }
    }

    current_nodes
        .iter()
        .fold(1, |multiple, &(_, steps)| lcm(multiple, steps))
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
    fn day08_part01_1() {
        let input = include_bytes!("input_test01.txt");
        assert_eq!(solve_part01(input), 2);
    }
    #[test]
    fn day08_part01_2() {
        let input = include_bytes!("input_test02.txt");
        assert_eq!(solve_part01(input), 6);
    }
    #[test]
    fn day08_part02() {
        let input = include_bytes!("input_test03.txt");
        assert_eq!(solve_part02(input), 6);
    }
}
