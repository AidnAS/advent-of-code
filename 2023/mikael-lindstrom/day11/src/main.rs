use std::collections::HashSet;

fn solve(input: &str, expansion: usize) -> u64 {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let galaxies = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, &ch)| {
                    if ch == '#' {
                        return Some((y, x));
                    }
                    None
                })
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();

    let empty_rows = (0..map.len())
        .enumerate()
        .filter_map(|(y, row)| {
            if map[row].iter().all(|&ch| ch == '.') {
                return Some(y);
            }
            None
        })
        .collect::<HashSet<_>>();

    let empty_columns = (0..map[0].len())
        .filter(|&x| map.iter().all(|row| row[x] == '.'))
        .collect::<HashSet<_>>();

    let mut sum_of_paths = 0;
    for (i, &(y1, x1)) in galaxies.iter().enumerate() {
        for &(y2, x2) in galaxies.iter().skip(i + 1) {
            let mut manhattan_distance = x1.abs_diff(x2) + y1.abs_diff(y2);
            for y in y1.min(y2)..y1.max(y2) {
                if empty_rows.contains(&y) {
                    manhattan_distance += expansion;
                }
            }
            for x in x1.min(x2)..x1.max(x2) {
                if empty_columns.contains(&x) {
                    manhattan_distance += expansion;
                }
            }
            sum_of_paths += manhattan_distance;
        }
    }
    sum_of_paths as u64
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", solve(input, 1));
    println!("part2: {}", solve(input, 999_999));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = include_str!("test_input.txt");
        assert_eq!(solve(input, 1), 374);
        assert_eq!(solve(input, 9), 1030);
        assert_eq!(solve(input, 99), 8410);
    }
}
