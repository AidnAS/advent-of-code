advent_of_code::solution!(11);
use std::collections::HashMap;

fn blink(stone: u64, remaining_blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if remaining_blinks == 0 {
        return 1;
    }
    if let Some(&result) = cache.get(&(stone, remaining_blinks)) {
        return result;
    }

    let result = if stone == 0 {
        blink(1, remaining_blinks - 1, cache)
    } else {
        let stone_string = stone.to_string();
        let is_even = stone_string.len() % 2 == 0;

        if is_even {
            let (lhs_str, rhs_str) = stone_string.split_at(stone_string.len() / 2);
            let lhs: u64 = lhs_str.parse().unwrap();
            let rhs: u64 = rhs_str.parse().unwrap();

            blink(lhs, remaining_blinks - 1, cache) + blink(rhs, remaining_blinks - 1, cache)
        } else {
            blink(stone * 2024, remaining_blinks - 1, cache)
        }
    };

    cache.insert((stone, remaining_blinks), result);
    result
}

fn count_stones(stones: Vec<u64>, remaining_blinks: u64) -> u64 {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|s| blink(*s, remaining_blinks, &mut cache))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones_vector = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let count = count_stones(stones_vector, 25);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones_vector = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let count = count_stones(stones_vector, 75);
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
