fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let (game, bag) = line.split_once(": ").unwrap();
            let game_id = game.trim_start_matches("Game ").parse::<u32>().unwrap();
            let valid = bag
                .split("; ")
                .map(|rounds| {
                    rounds.split(", ").fold(true, |valid, round| {
                        match round.split_once(" ").unwrap() {
                            (amount, "red") => valid && amount.parse::<u32>().unwrap() <= 12,
                            (amount, "green") => valid && amount.parse::<u32>().unwrap() <= 13,
                            (amount, "blue") => valid && amount.parse::<u32>().unwrap() <= 14,
                            _ => valid,
                        }
                    })
                })
                .all(|v| v == true);
            if valid {
                return Some(game_id);
            }
            None
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split(": ")
                .skip(1)
                .next()
                .unwrap()
                .split([';', ','])
                .fold([0, 0, 0], |[mut r, mut g, mut b], color| {
                    match color.trim().split_once(" ").unwrap() {
                        (amount, "red") => r = amount.parse::<u32>().unwrap().max(r),
                        (amount, "green") => g = amount.parse::<u32>().unwrap().max(g),
                        (amount, "blue") => b = amount.parse::<u32>().unwrap().max(b),
                        _ => (),
                    }
                    [r, g, b]
                })
                .iter()
                .product::<u32>()
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
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 2286);
    }
}
