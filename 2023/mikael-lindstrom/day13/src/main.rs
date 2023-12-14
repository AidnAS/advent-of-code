fn check(pattern: &[&str], i: usize) -> bool {
    for j in 0..=i {
        let mirror_j = i + ((i + 1) - j);
        if mirror_j >= pattern.len() {
            continue;
        }
        if pattern[j] != pattern[mirror_j] {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let pattern = pattern.lines().collect::<Vec<_>>();
            if let Some(index) = (0..pattern.len() - 1).find(|&i| check(&pattern, i)) {
                return 100 * (index + 1);
            }
            let transposed = (0..pattern[0].len())
                .map(|column| {
                    pattern
                        .iter()
                        .map(|row| row.chars().nth(column).unwrap())
                        .collect::<String>()
                })
                .collect::<Vec<_>>();
            let transposed: Vec<&str> = transposed.iter().map(|s| s.as_str()).collect();
            if let Some(index) = (0..transposed.len() - 1).find(|&i| check(&transposed, i)) {
                return index + 1;
            }
            0
        })
        .sum::<usize>()
}

fn check_smudge(pattern: &[&str], i: usize) -> bool {
    let mut smudges = 0;
    for j in 0..=i {
        let mirror_j = i + ((i + 1) - j);
        if mirror_j >= pattern.len() {
            continue;
        }
        smudges += pattern[j]
            .chars()
            .zip(pattern[mirror_j].chars())
            .filter(|(a, b)| a != b)
            .count();
    }
    smudges == 1
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let pattern = pattern.lines().collect::<Vec<_>>();
            if let Some(index) = (0..pattern.len() - 1).find(|&i| check_smudge(&pattern, i)) {
                return 100 * (index + 1);
            }
            let transposed = (0..pattern[0].len())
                .map(|column| {
                    pattern
                        .iter()
                        .map(|row| row.chars().nth(column).unwrap())
                        .collect::<String>()
                })
                .collect::<Vec<_>>();
            let transposed: Vec<&str> = transposed.iter().map(|s| s.as_str()).collect();
            if let Some(index) = (0..transposed.len() - 1).find(|&i| check_smudge(&transposed, i)) {
                return index + 1;
            }
            0
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
        assert_eq!(part1(input), 405);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 400);
    }
}
