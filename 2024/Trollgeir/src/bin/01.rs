advent_of_code::solution!(1);

fn parse_locations(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (locations_1, locations_2) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            Some((
                parts.next()?.parse::<u32>().ok()?,
                parts.next()?.parse::<u32>().ok()?,
            ))
        })
        .unzip();

    (locations_1, locations_2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut locations_1, mut locations_2) = parse_locations(input);

    locations_1.sort_unstable();
    locations_2.sort_unstable();

    let result = locations_1
        .into_iter()
        .zip(locations_2)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (locations_1, locations_2) = parse_locations(input);

    let value_counts =
        locations_2
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut acc, loc| {
                *acc.entry(loc).or_insert(0) += 1;
                acc
            });

    let result = locations_1
        .into_iter()
        .map(|loc| loc * value_counts.get(&loc).unwrap_or(&0))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
