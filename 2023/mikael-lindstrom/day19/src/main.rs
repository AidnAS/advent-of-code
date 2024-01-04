use std::collections::HashMap;

fn parse_workflows(workflows: &str) -> HashMap<String, Vec<(char, char, usize, String)>> {
    workflows.lines().fold(HashMap::new(), |mut acc, line| {
        let (instruction, rules) = line.trim_end_matches('}').split_once('{').unwrap();
        let rules = rules
            .split(',')
            .map(|rule| {
                if rule.contains(':') {
                    let mut chars = rule.chars();
                    let part = chars.next().unwrap();
                    let operation = chars.next().unwrap();
                    let rest = chars.collect::<String>();
                    let (limit, destination) = rest.split_once(':').unwrap();
                    (
                        part,
                        operation,
                        limit.parse::<usize>().unwrap(),
                        destination.to_string(),
                    )
                } else {
                    ('_', '=', 0, rule.to_string())
                }
            })
            .collect::<Vec<_>>();
        acc.insert(instruction.to_string(), rules);
        acc
    })
}

fn part1(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);

    parts
        .lines()
        .map(|line| {
            let x: &[_] = &['{', '}'];
            line.trim_matches(x)
                .split(',')
                .fold(HashMap::new(), |mut acc, part| {
                    let (p, value) = part.split_once('=').unwrap();
                    acc.insert(p.chars().next().unwrap(), value.parse::<usize>().unwrap());
                    acc
                })
        })
        .filter_map(|part| {
            let mut current = "in";
            loop {
                if current == "A" {
                    return Some(part.values().sum::<usize>());
                }
                if current == "R" {
                    return None;
                }
                for rule in workflows.get(current).unwrap() {
                    match rule {
                        (p, '<', value, dest) => {
                            let ch = part.get(p).unwrap();
                            if ch < value {
                                current = dest;
                                break;
                            }
                        }
                        (p, '>', value, dest) => {
                            let ch = part.get(p).unwrap();
                            if ch > value {
                                current = dest;
                                break;
                            }
                        }
                        (_, '=', _, dest) => {
                            current = dest;
                        }
                        _ => unreachable!(),
                    }
                }
            }
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);

    let valid_ranges = HashMap::from([
        ('x', (1..=4_000)),
        ('m', (1..=4_000)),
        ('a', (1..=4_000)),
        ('s', (1..=4_000)),
    ]);

    let mut queue = vec![("in", valid_ranges)];
    let mut accepted = Vec::new();
    while let Some((workflow, mut ranges)) = queue.pop() {
        if workflow == "A" {
            accepted.push(ranges);
            continue;
        }
        if workflow == "R" {
            continue;
        }
        for rule in workflows.get(workflow).unwrap() {
            match rule {
                (part, '<', value, dest) => {
                    let range = ranges.get(part).unwrap();
                    if range.end() < value {
                        queue.push((dest, ranges.clone()));
                    } else if range.start() < value {
                        let mut new_ranges = ranges.clone();
                        *new_ranges.get_mut(part).unwrap() = *range.start()..=*value - 1;
                        queue.push((dest, new_ranges));
                        *ranges.get_mut(part).unwrap() = *value..=*range.end();
                    }
                }
                (part, '>', value, dest) => {
                    let range = ranges.get(part).unwrap();
                    if range.start() > value {
                        queue.push((dest, ranges.clone()));
                    } else if range.end() > value {
                        let mut new_ranges = ranges.clone();
                        *new_ranges.get_mut(part).unwrap() = *value + 1..=*range.end();
                        queue.push((dest, new_ranges));
                        *ranges.get_mut(part).unwrap() = *range.start()..=*value;
                    }
                }
                (_, '=', _, destination) => {
                    queue.push((destination, ranges.clone()));
                }
                _ => {}
            }
        }
    }

    accepted
        .iter()
        .map(|ranges| {
            ranges.iter().fold(1, |mut acc, (_, value)| {
                acc *= value.end() - value.start() + 1;
                acc
            })
        })
        .sum::<usize>()
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
        assert_eq!(part1(input), 19114);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 167409079868000);
    }
}
