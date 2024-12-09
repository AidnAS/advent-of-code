advent_of_code::solution!(4);

const XMAS_REGEX: &str = r"(XMAS|SAMX)";
const MAS_REGEX: &str = r"(MAS|SAM)";

fn scan_for_overlapping_pattern(input: &str, pattern: &str) -> Vec<(usize, usize)> {
    let re = regex::Regex::new(pattern).unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            let mut positional_hits = Vec::new();
            let mut search_start = 0;
            while let Some(mat) = re.find(&line[search_start..]) {
                positional_hits.push((line_index, search_start + mat.start() + 1));
                search_start += mat.start() + 1;
            }
            positional_hits
        })
        .collect::<Vec<_>>()
}

fn rotate_90_deg(input: &str) -> String {
    let v: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dims = v.len();

    let mut new_v: Vec<Vec<char>> = vec![vec![' '; dims]; dims];

    for (i, row) in v.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            new_v[j][dims - 1 - i] = val;
        }
    }
    new_v
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn rotate_45_deg(input: &str) -> String {
    let v: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dims = v.len();

    let mut new_v: Vec<Vec<char>> = vec![vec![' '; dims + dims - 1]; dims + dims - 1];

    for i in 0..dims {
        for j in 0..dims {
            new_v[i + j][j] = v[i][j];
        }
    }
    new_v
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = scan_for_overlapping_pattern(input, XMAS_REGEX).len() as u32;

    let input_rotated_45 = rotate_45_deg(input);
    count += scan_for_overlapping_pattern(&input_rotated_45, XMAS_REGEX).len() as u32;

    let input_rotated_90 = rotate_90_deg(input);
    count += scan_for_overlapping_pattern(&input_rotated_90, XMAS_REGEX).len() as u32;

    let input_rotated_135 = rotate_45_deg(&input_rotated_90);
    count += scan_for_overlapping_pattern(&input_rotated_135, XMAS_REGEX).len() as u32;

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_rotated_45 = rotate_45_deg(input);
    let hits_45 = scan_for_overlapping_pattern(&input_rotated_45, MAS_REGEX);

    let input_rotated_90 = rotate_90_deg(input);
    let input_rotated_135 = rotate_45_deg(&input_rotated_90);
    let hits_135 = scan_for_overlapping_pattern(&input_rotated_135, MAS_REGEX);

    let dim_size = input.lines().count();

    let count = hits_45
        .iter()
        .filter(|&&(i, j)| {
            let d_hit_normalized = (i - j, j);
            hits_135.iter().any(|&(k, l)| {
                let d_hit_f_normalized = (dim_size - 1 - l, k - l);
                d_hit_normalized == d_hit_f_normalized
            })
        })
        .count() as u32;
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
