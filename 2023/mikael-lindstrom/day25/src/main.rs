use std::collections::{HashMap, HashSet, VecDeque};

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let (node, neighbours) = line.split_once(':').unwrap();
        let neighbours = neighbours.split_whitespace().collect::<HashSet<_>>();
        neighbours.iter().for_each(|neighbour| {
            acc.entry(neighbour).or_default().insert(node);
        });
        acc.entry(node).or_default().extend(neighbours);
        acc
    })
}

fn remove_most_frequent_edge<'a>(nodes: &mut HashMap<&'a str, HashSet<&'a str>>) {
    let mut frequency = HashMap::new();
    for &start_node in nodes.keys() {
        let mut queue = VecDeque::new();
        queue.push_back(start_node);
        let mut seen = HashSet::new();
        seen.insert(start_node);

        while let Some(current_node) = queue.pop_front() {
            // This is done to force a new random path for each interation, otherwise there will
            // be "hot" paths and the statistics gets wrong
            for &neighbour in
                nodes
                    .get(current_node)
                    .unwrap()
                    .iter()
                    .fold(HashSet::new(), |mut acc, value| {
                        acc.insert(value);
                        acc
                    })
            {
                if seen.insert(neighbour) {
                    let edge = if current_node < neighbour {
                        (current_node, neighbour)
                    } else {
                        (neighbour, current_node)
                    };
                    *frequency.entry(edge).or_insert(0) += 1;
                    queue.push_back(neighbour);
                }
            }
        }
    }
    let mut frequency = frequency.into_iter().collect::<Vec<_>>();
    frequency.sort_by_key(|o| o.1);
    frequency.reverse();
    let most_frequent_edge = frequency.first().unwrap();
    let ((first_node, second_node), _) = most_frequent_edge;
    nodes.get_mut(first_node).unwrap().remove(second_node);
    nodes.get_mut(second_node).unwrap().remove(first_node);
}

fn get_answer(nodes: &HashMap<&str, HashSet<&str>>) -> usize {
    let (&start_node, _) = nodes.iter().next().unwrap();
    let mut queue = VecDeque::new();
    queue.push_back(start_node);
    let mut seen = HashSet::new();
    seen.insert(start_node);

    while let Some(current_node) = queue.pop_front() {
        for &neighbour in nodes.get(current_node).unwrap() {
            if seen.insert(neighbour) {
                queue.push_back(neighbour);
            }
        }
    }
    seen.len() * (nodes.len() - seen.len())
}

fn part1(input: &str) -> usize {
    let mut nodes = parse(input);
    let mut answer = 0;
    // Since the algorithm uses random values there is a small possibility that the statistics gets
    // wrong and we cut the wrong edges. If so, the answer will be zero and we run it again.
    while answer == 0 {
        remove_most_frequent_edge(&mut nodes);
        remove_most_frequent_edge(&mut nodes);
        remove_most_frequent_edge(&mut nodes);
        answer = get_answer(&nodes);
    }
    answer
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {:?}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input), 54);
    }
}
