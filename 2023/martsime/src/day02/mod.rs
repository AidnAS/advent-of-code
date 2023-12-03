use crate::solution::Solution;

pub fn solve_part01(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (title, game) = line.split_once(": ").unwrap();
            let game_number = title.trim_start_matches("Game ").parse::<u32>().unwrap();
            let possible = game.split("; ").all(|selection| {
                selection.split(", ").all(|cube_string| {
                    match cube_string.split_once(' ').unwrap() {
                        (count, "red") => count.parse::<u32>().unwrap() <= 12,
                        (count, "green") => count.parse::<u32>().unwrap() <= 13,
                        (count, "blue") => count.parse::<u32>().unwrap() <= 14,
                        (_, cube) => {
                            panic!("Invalid cube: {}", cube);
                        }
                    }
                })
            });
            game_number * (possible as u32)
        })
        .sum::<u32>()
}

pub fn solve_part02(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_title, game) = line.split_once(": ").unwrap();
            let (mut red, mut green, mut blue) = (0, 0, 0);
            game.split("; ").for_each(|selection| {
                selection.split(", ").for_each(|cube_string| {
                    match cube_string.split_once(' ').unwrap() {
                        (count, "red") => red = red.max(count.parse::<u32>().unwrap()),
                        (count, "green") => green = green.max(count.parse::<u32>().unwrap()),
                        (count, "blue") => blue = blue.max(count.parse::<u32>().unwrap()),
                        (_, cube) => {
                            panic!("Invalid cube: {}", cube);
                        }
                    }
                })
            });
            red * green * blue
        })
        .sum::<u32>()
}

pub fn part01() -> Solution {
    solve_part01(include_str!("input.txt")).into()
}

pub fn part02() -> Solution {
    solve_part02(include_str!("input.txt")).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_part01() {
        let input = include_str!("input_test.txt");
        let solution = solve_part01(input);
        assert_eq!(solution, 8);
    }

    #[test]
    fn day02_part02() {
        let input = include_str!("input_test.txt");
        let solution = solve_part02(input);
        assert_eq!(solution, 2286);
    }
}
