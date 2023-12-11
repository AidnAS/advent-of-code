use crate::solution::Solution;

fn solve(input: &[u8], expansion: i64) -> i64 {
    let mut width = 0;
    let mut galaxies = input
        .split(|&b| b == b'\n')
        .enumerate()
        .flat_map(|(y, line)| {
            width = line.len();
            line.iter().enumerate().filter_map(move |(x, &b)| match b {
                b'#' => Some((x, y, x as i64, y as i64)),
                _ => None,
            })
        })
        .collect::<Vec<_>>();
    let height = input.len() / width;

    for x in 0..width {
        if !galaxies.iter().any(|(gx, _, _, _)| *gx == x) {
            galaxies
                .iter_mut()
                .filter(|(gx, _, _, _)| *gx > x)
                .for_each(|(_, _, offset_x, _)| *offset_x += expansion - 1);
        }
    }
    for y in 0..height {
        if !galaxies.iter().any(|(_, gy, _, _)| *gy == y) {
            galaxies
                .iter_mut()
                .filter(|(_, gy, _, _)| *gy > y)
                .for_each(|(_, _, _, offset_y)| *offset_y += expansion - 1);
        }
    }

    let total_distance = galaxies
        .iter()
        .enumerate()
        .flat_map(|(index, (_, _, x1, y1))| {
            galaxies[index + 1..]
                .iter()
                .map(move |(_, _, x2, y2)| (x2 - x1).abs() + (y2 - y1).abs())
        })
        .sum();

    total_distance
}

pub fn part01() -> Solution {
    solve(include_bytes!("input.txt"), 2).into()
}

pub fn part02() -> Solution {
    solve(include_bytes!("input.txt"), 1_000_000).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_part01() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve(input, 2), 374);
    }

    #[test]
    fn day11_part02_1() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve(input, 10), 1030);
    }
    #[test]
    fn day11_part02_2() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve(input, 100), 8410);
    }
}
