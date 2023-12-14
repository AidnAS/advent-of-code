use std::collections::HashMap;

fn parse_part1(input: &str) -> Vec<(String, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (spring, groups) = line.split_once(' ').unwrap();
            let spring = spring.to_string();
            let groups = groups
                .split(',')
                .map(|group| group.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (spring, groups)
        })
        .collect::<Vec<_>>()
}

fn parse_part2(input: &str) -> Vec<(String, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (spring, groups) = line.split_once(' ').unwrap();
            let spring = (0..5).map(|_| spring).collect::<Vec<&str>>().join("?");
            let groups = groups
                .split(',')
                .map(|group| group.parse::<usize>().unwrap())
                .cycle()
                .take(5 * groups.split(',').count())
                .clone()
                .collect::<Vec<_>>();
            (spring, groups)
        })
        .collect::<Vec<_>>()
}

fn solve<'a>(
    spring: &'a str,
    groups: &'a [usize],
    cache: &mut HashMap<(&'a str, &'a [usize]), usize>,
) -> usize {
    if groups.is_empty() {
        if spring.contains('#') {
            return 0;
        }
        return 1;
    }
    if spring.is_empty() {
        return 0;
    }
    if let Some(&result) = cache.get(&(spring, groups)) {
        return result;
    }

    let mut result = 0;
    if spring.starts_with('.') || spring.starts_with('?') {
        result += solve(&spring[1..], groups, cache);
    }
    if (spring.starts_with('#') || spring.starts_with('?'))
        && groups[0] <= spring.len()
        && !spring.chars().take(groups[0]).any(|c| c == '.')
    {
        if groups[0] == spring.len() {
            result += solve(&spring[groups[0]..], &groups[1..], cache);
        } else if spring.chars().nth(groups[0]) != Some('#') {
            result += solve(&spring[groups[0] + 1..], &groups[1..], cache);
        }
    }
    cache.insert((spring, groups), result);
    result
}

fn part1(input: &str) -> usize {
    parse_part1(input)
        .iter()
        .map(|(spring, groups)| {
            let mut cache = HashMap::new();
            solve(spring.as_str(), groups, &mut cache)
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    parse_part2(input)
        .iter()
        .map(|(spring, groups)| {
            let mut cache = HashMap::new();
            solve(spring, groups, &mut cache)
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
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 525152);
    }
}
