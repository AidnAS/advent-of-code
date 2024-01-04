fn get_volume(plan: &Vec<(&str, i64)>) -> i64 {
    let mut y1 = 0;
    let mut y2 = 0;
    let mut x1 = 0;
    let mut x2 = 0;
    let mut border = 0;
    let mut area = 0;

    for (direction, steps) in plan {
        match *direction {
            "U" => y2 -= steps,
            "R" => x2 += steps,
            "D" => y2 += steps,
            "L" => x2 -= steps,
            _ => unreachable!(),
        }
        // Shoelace formula
        area += (y1 + y2) * (x1 - x2);

        border += steps;
        y1 = y2;
        x1 = x2;
    }
    // Pick's theorem
    (area.abs() + border) / 2 + 1
}

fn part1(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|line| {
            let line = line.split(' ').collect::<Vec<_>>();
            (line[0], line[1].parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();
    get_volume(&instructions) as usize
}

fn part2(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|line| {
            let (steps, direction) = line
                .split_once('#')
                .unwrap()
                .1
                .trim_end_matches(')')
                .split_at(5);
            let direction = match direction.parse::<usize>().unwrap() {
                0 => "R",
                1 => "D",
                2 => "L",
                3 => "U",
                _ => unreachable!(),
            };
            let steps = i64::from_str_radix(steps, 16).unwrap();
            (direction, steps)
        })
        .collect::<Vec<_>>();
    get_volume(&instructions)
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input), 62);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 952408144115);
    }
}
