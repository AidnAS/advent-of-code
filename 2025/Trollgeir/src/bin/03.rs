advent_of_code::solution!(3);

fn deduce_line<const NUM_CAPTURES: usize>(line: &str) -> u64 {
    let bytes = line.as_bytes();
    let mut stack: Vec<u32> = Vec::with_capacity(NUM_CAPTURES);
    let mut to_drop = bytes.len() - NUM_CAPTURES;

    for &byte in bytes {
        let digit = (byte - b'0') as u32;
        while to_drop > 0 {
            if let Some(&last) = stack.last() {
                if last >= digit {
                    break;
                }
                stack.pop();
                to_drop -= 1;
            } else {
                break;
            }
        }
        stack.push(digit);
    }
    stack
        .iter()
        .take(NUM_CAPTURES)
        .fold(0u64, |acc, &digit| acc * 10 + digit as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    const NUM_CAPTURES: usize = 2;
    Some(input.lines().map(deduce_line::<NUM_CAPTURES>).sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    const NUM_CAPTURES: usize = 12;
    Some(input.lines().map(deduce_line::<NUM_CAPTURES>).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
