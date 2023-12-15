use std::collections::HashMap;

fn tilt(platform: &mut Vec<Vec<char>>) {
    for _ in 0..platform.len() - 1 {
        for row in 0..platform.len() - 1 {
            for column in 0..platform[row].len() {
                if platform[row + 1][column] == 'O' && platform[row][column] == '.' {
                    platform[row][column] = 'O';
                    platform[row + 1][column] = '.';
                }
            }
        }
    }
}
#[allow(clippy::needless_range_loop)]
fn rotate(platform: &mut Vec<Vec<char>>) {
    let mut transposed = vec![vec![' '; platform.len()]; platform[0].len()];
    for j in 0..platform.len() {
        for i in 0..platform[0].len() {
            transposed[j][i] = platform[i][j];
        }
    }
    for row in &mut transposed {
        row.reverse();
    }
    *platform = transposed;
}

fn part1(input: &str) -> usize {
    let mut platform = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    tilt(&mut platform);
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&column| column == &'O').count() * (platform.len() - i))
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let mut platform = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut cache = HashMap::new();
    for i in 0..999_000_000 {
        for _ in 0..4 {
            tilt(&mut platform);
            rotate(&mut platform);
        }
        if let Some(hit) = cache.insert(platform.clone(), i) {
            if 0 == (999_000_000 - i) % (i - hit) {
                break;
            }
        }
    }
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&column| column == &'O').count() * (platform.len() - i))
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
        assert_eq!(part1(input), 136);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 64);
    }
}
