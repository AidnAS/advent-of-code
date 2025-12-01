advent_of_code::solution!(1);

fn parse_line(line: &str) -> i32 {
    let (direction, distance) = line.split_at(1);
    let distance: i32 = distance.parse().unwrap();
    match direction {
        "R" => distance,
        "L" => -distance,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut dial_cursor = 50;
    let zeroes = input
        .lines()
        .map(parse_line)
        .map(|rotation| {
            dial_cursor = (dial_cursor + rotation).rem_euclid(100);
            u32::from(dial_cursor == 0)
        })
        .sum();
    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut dial_cursor = 50;
    let total_zero_clicks: u32 = input
        .lines()
        .map(parse_line)
        .map(|rotation| {
            let end = dial_cursor + rotation;
            let shift = -i32::from(rotation.is_negative());
            let zero_clicks = (end + shift)
                .div_euclid(100)
                .abs_diff((dial_cursor + shift).div_euclid(100));
            dial_cursor = end.rem_euclid(100);
            zero_clicks
        })
        .sum();
    Some(total_zero_clicks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
