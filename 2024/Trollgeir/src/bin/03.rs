advent_of_code::solution!(3);

const MUL_REGEX: &str = r"mul\((\d{1,3}),(\d{1,3})\)";

pub fn part_one(input: &str) -> Option<u32> {
    let result = regex::Regex::new(MUL_REGEX)
        .unwrap()
        .captures_iter(input)
        .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
        .sum();
    Some(result)
}

const MUL_REGEX_V2: &str = r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))";

pub fn part_two(input: &str) -> Option<u32> {
    let result = regex::Regex::new(MUL_REGEX_V2)
        .unwrap()
        .captures_iter(input)
        .fold((true, 0), |mut acc, cap| {
            match &cap[0] {
                "do()" => acc.0 = true,
                "don't()" => acc.0 = false,
                _ => {
                    let x = cap[2].parse::<u32>().unwrap();
                    let y = cap[3].parse::<u32>().unwrap();
                    if acc.0 {
                        acc.1 += x * y;
                    }
                }
            }
            acc
        });

    Some(result.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
