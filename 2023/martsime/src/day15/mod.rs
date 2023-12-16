use crate::solution::Solution;
use crate::utils::parse_number_from_bytes;

fn hash(bytes: &[u8]) -> u8 {
    let mut value: u32 = 0;
    for &byte in bytes {
        value = ((value + byte as u32) * 17) % 256;
    }
    debug_assert!(value < 256);
    value as u8
}

fn solve_part01(input: &[u8]) -> u32 {
    input
        .split(|&b| b == b',')
        .map(|slice| hash(slice) as u32)
        .sum::<u32>()
}

fn solve_part02(input: &[u8]) -> u32 {
    let mut boxes: Vec<Vec<(&[u8], u32)>> = vec![vec![]; 256];
    input.split(|&b| b == b',').for_each(|bytes| {
        let sign_index = bytes.iter().position(|&b| b == b'=' || b == b'-').unwrap();
        let label = &bytes[..sign_index];
        let box_number = hash(&bytes[..sign_index]);
        let sign = bytes[sign_index];

        let box_lenses = &mut boxes[box_number as usize];
        match sign {
            b'-' => box_lenses.retain(|&(lense, _)| lense != label),
            b'=' => {
                let value = parse_number_from_bytes::<u32>(&bytes[sign_index + 1..]);
                let existing_index = box_lenses.iter().position(|&(lense, _)| lense == label);
                match existing_index {
                    Some(index) => box_lenses[index].1 = value,
                    None => box_lenses.push((label, value)),
                }
            }
            _ => unreachable!(),
        }
    });

    boxes
        .iter()
        .enumerate()
        .filter(|(_, lenses)| !lenses.is_empty())
        .map(|(box_index, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lense_index, &(_, focal_length))| {
                    (box_index + 1) as u32 * (lense_index + 1) as u32 * focal_length
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

pub fn part01() -> Solution {
    solve_part01(include_bytes!("input.txt")).into()
}

pub fn part02() -> Solution {
    solve_part02(include_bytes!("input.txt")).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part01() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part01(input), 1320);
    }

    #[test]
    fn day15_part02() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part02(input), 145);
    }
}
