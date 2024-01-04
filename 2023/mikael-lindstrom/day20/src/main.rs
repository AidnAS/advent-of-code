use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
enum Modules<'a> {
    Broadcaster {
        destinations: HashSet<&'a str>,
    },
    Conjunction {
        destinations: HashSet<&'a str>,
        state: HashMap<&'a str, bool>,
    },
    FlipFlop {
        destinations: HashSet<&'a str>,
        state: bool,
    },
}

fn parse(input: &str) -> HashMap<&str, Modules<'_>> {
    let mut modules = input
        .lines()
        .map(|line| {
            let (name, destinations) = line.split_once("->").unwrap();
            let destinations = destinations
                .split(',')
                .map(|line| line.trim())
                .collect::<HashSet<&str>>();
            match line.chars().next().unwrap() {
                '%' => {
                    let name = name.trim().trim_start_matches('%');
                    let module = Modules::FlipFlop {
                        destinations,
                        state: false,
                    };
                    (name, module)
                }
                '&' => {
                    let name = name.trim().trim_start_matches('&');
                    let module = Modules::Conjunction {
                        destinations,
                        state: HashMap::new(),
                    };
                    (name, module)
                }
                'b' => {
                    let name = name.trim().trim_start_matches('%');
                    let module = Modules::Broadcaster { destinations };
                    (name, module)
                }
                _ => unreachable!(),
            }
        })
        .fold(HashMap::new(), |mut acc, (name, module)| {
            acc.insert(name, module);
            acc
        });

    for (name, module) in modules.clone().iter() {
        let destinations = match module {
            Modules::Broadcaster { destinations } => destinations,
            Modules::FlipFlop { destinations, .. } => destinations,
            Modules::Conjunction { destinations, .. } => destinations,
        };
        for destination in destinations {
            if let Some(Modules::Conjunction { state, .. }) = modules.get_mut(destination) {
                state.insert(name, false);
            }
        }
    }
    modules
}

fn part1(input: &str) -> usize {
    let mut modules = parse(input);

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1_000 {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", false));
        while let Some((from, to, pulse)) = queue.pop_front() {
            if pulse {
                high_count += 1;
            } else {
                low_count += 1;
            }
            if let Some(module) = modules.get_mut(to) {
                match module {
                    Modules::Broadcaster { destinations } => {
                        destinations
                            .iter()
                            .for_each(|destination| queue.push_back((to, destination, pulse)));
                    }
                    Modules::FlipFlop {
                        destinations,
                        state,
                    } => {
                        if pulse {
                            continue;
                        }
                        *state = !*state;
                        if *state {
                            destinations
                                .iter()
                                .for_each(|destination| queue.push_back((to, destination, true)));
                        } else {
                            destinations
                                .iter()
                                .for_each(|destination| queue.push_back((to, destination, false)));
                        }
                    }
                    Modules::Conjunction {
                        destinations,
                        state,
                    } => {
                        *state.get_mut(from).unwrap() = pulse;
                        if state.values().all(|pulse| *pulse) {
                            destinations
                                .iter()
                                .for_each(|destination| queue.push_back((to, destination, false)));
                        } else {
                            destinations
                                .iter()
                                .for_each(|destination| queue.push_back((to, destination, true)));
                        };
                    }
                }
            }
        }
    }
    high_count * low_count
}

fn part2(input: &str) -> usize {
    let mut modules = parse(input);
    let (_, destination_rx_module) = modules
        .iter()
        .find(|(_, module)| match module {
            Modules::Broadcaster { destinations } => destinations.contains("rx"),
            Modules::FlipFlop { destinations, .. } => destinations.contains("rx"),
            Modules::Conjunction { destinations, .. } => destinations.contains("rx"),
        })
        .unwrap();
    let Modules::Conjunction { state, .. } = destination_rx_module else {
        unreachable!();
    };
    let mut sub_cycle = state.iter().fold(HashMap::new(), |mut acc, (key, _)| {
        acc.insert(key.to_string(), 0);
        acc
    });
    for i in 1..10000 {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", false));
        while let Some((from, to, pulse)) = queue.pop_front() {
            if let Some(module) = modules.get_mut(to) {
                match module {
                    Modules::Broadcaster { destinations } => {
                        destinations
                            .iter()
                            .for_each(|destination| queue.push_back((to, destination, pulse)));
                    }
                    Modules::FlipFlop {
                        destinations,
                        state,
                    } => {
                        if pulse {
                            continue;
                        }
                        *state = !*state;
                        if *state {
                            destinations
                                .iter()
                                .for_each(|destination| queue.push_back((to, destination, true)));
                        } else {
                            destinations
                                .iter()
                                .for_each(|destination| queue.push_back((to, destination, false)));
                        }
                    }
                    Modules::Conjunction {
                        destinations,
                        state,
                    } => {
                        *state.get_mut(from).unwrap() = pulse;
                        if state.values().all(|pulse| *pulse) {
                            destinations.iter().for_each(|destination| {
                                if let Some(value) = sub_cycle.get_mut(*destination) {
                                    if *value == 0 {
                                        *value = i;
                                    }
                                }
                                queue.push_back((to, destination, false))
                            });
                        } else {
                            destinations
                                .iter()
                                .for_each(|destination| queue.push_back((to, destination, true)));
                        };
                    }
                }
            }
        }
    }
    sub_cycle.iter().fold(1, |mut acc, (_, value)| {
        acc = lcm(acc, value);
        acc
    })
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: &usize) -> usize {
    (a * b) / gcd(a, *b)
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
        assert_eq!(part1(input_a), 32_000_000);
        let input_b = include_str!("test_input_1b.txt");
        assert_eq!(part1(input_b), 11_687_500);
    }
}
