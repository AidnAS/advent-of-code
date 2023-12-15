use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let (instructions_str, network_str) = input.split_once("\n\n").unwrap();
    let instructions = instructions_str.chars().collect::<Vec<_>>();
    let network = network_str.lines().fold(HashMap::new(), |mut acc, line| {
        let node = line
            .split(['\n', ' ', '=', '(', ')', ','])
            .filter(|&value| !value.is_empty())
            .collect::<Vec<_>>();
        acc.insert(node[0], (node[1], node[2]));
        acc
    });
    let mut current = "AAA";
    let mut count = 0;
    for instruction in instructions.iter().cycle() {
        if *instruction == 'L' {
            current = network.get(current).unwrap().0;
        } else {
            current = network.get(current).unwrap().1;
        }
        count += 1;
        if current == "ZZZ" {
            break;
        }
    }
    count
}

fn part2(input: &str) -> u64 {
    let (instructions_str, network_str) = input.split_once("\n\n").unwrap();
    let instructions = instructions_str.chars().collect::<Vec<_>>();
    let network = network_str.lines().fold(HashMap::new(), |mut acc, line| {
        let node = line
            .split(['\n', ' ', '=', '(', ')', ','])
            .filter(|&value| !value.is_empty())
            .collect::<Vec<_>>();
        acc.insert(node[0], (node[1], node[2]));
        acc
    });
    let starting_nodes = network
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();
    starting_nodes
        .iter()
        .map(|&&(mut node)| {
            let mut count: u64 = 0;
            for instruction in instructions.iter().cycle() {
                if *instruction == 'L' {
                    node = network.get(node).unwrap().0;
                } else {
                    node = network.get(node).unwrap().1;
                }
                count += 1;
                if node.ends_with('Z') {
                    break;
                }
            }
            count
        })
        .fold(1, lcm)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_a = include_str!("test_input_1a.txt");
        assert_eq!(part1(input_a), 2);
        let input_b = include_str!("test_input_1b.txt");
        assert_eq!(part1(input_b), 6);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input_2.txt");
        assert_eq!(part2(input), 6);
    }
}
