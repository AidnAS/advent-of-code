fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|numbers| {
            format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .fold(Vec::new(), |mut acc, (index, c)| {
                    if c.is_ascii_digit() {
                        acc.push(c.to_digit(10).unwrap());
                    } else {
                        let substring = &line[index..];
                        match substring {
                            s if s.starts_with("one") => acc.push(1),
                            s if s.starts_with("two") => acc.push(2),
                            s if s.starts_with("three") => acc.push(3),
                            s if s.starts_with("four") => acc.push(4),
                            s if s.starts_with("five") => acc.push(5),
                            s if s.starts_with("six") => acc.push(6),
                            s if s.starts_with("seven") => acc.push(7),
                            s if s.starts_with("eight") => acc.push(8),
                            s if s.starts_with("nine") => acc.push(9),
                            _ => (),
                        }
                    }
                    acc
                })
        })
        .map(|numbers| {
            format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
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
        let input = include_str!("test_input_1.txt");
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input_2.txt");
        assert_eq!(part2(input), 281);
    }
}
