use std::str::from_utf8;

use crate::solution::Solution;
use crate::utils::parse_number_from_bytes;

type Vector = (i64, i64);

fn parse_line_part01(line: &[u8]) -> Vector {
    let mut line_iter = line.split(|&b| b == b' ');
    let direction = line_iter.next().unwrap()[0];
    let length = parse_number_from_bytes::<i64>(line_iter.next().unwrap());

    match direction {
        b'U' => (0, -length),
        b'L' => (-length, 0),
        b'D' => (0, length),
        b'R' => (length, 0),
        _ => unreachable!(),
    }
}

fn parse_line_part02(line: &[u8]) -> Vector {
    let mut line_iter = line.split(|&b| b == b' ').skip(2);
    let color = &line_iter.next().unwrap()[2..8];
    let length = i64::from_str_radix(from_utf8(&color[..5]).unwrap(), 16).unwrap();
    let direction = color[5];

    match direction {
        b'0' => (length, 0),
        b'1' => (0, length),
        b'2' => (-length, 0),
        b'3' => (0, -length),
        _ => unreachable!(),
    }
}

fn solve(input: &[u8], line_parser: fn(&[u8]) -> Vector) -> i64 {
    let (area, _, _) = input.split(|&b| b == b'\n').map(line_parser).fold(
        (0, 0, 0),
        |(area, x1, y1), (dx, dy)| {
            let (x2, y2) = (x1 + dx, y1 + dy);
            // Shoelace formula for interior area plus the circumference
            let new_area = area + x1 * y2 - x2 * y1 + dx.abs() + dy.abs();
            (new_area, x2, y2)
        },
    );
    area / 2 + 1
}

pub fn part01() -> Solution {
    solve(include_bytes!("input.txt"), parse_line_part01).into()
}

pub fn part02() -> Solution {
    solve(include_bytes!("input.txt"), parse_line_part02).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_part01() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve(input, parse_line_part01), 62);
    }

    #[test]
    fn day18_part02() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve(input, parse_line_part02), 952408144115);
    }
}
