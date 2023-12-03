use std::ops::RangeInclusive;

struct AdjacentRange {
    x: RangeInclusive<usize>,
    y: usize,
}

struct PartNumber {
    number: String,
    x: usize,
    y: usize,
    width: usize,
}

impl PartNumber {
    fn is_adjacent(&self, gear: &Gear) -> bool {
        [
            AdjacentRange {
                x: self.x.saturating_sub(1)..=self.x.saturating_add(self.width),
                y: self.y.saturating_sub(1),
            },
            AdjacentRange {
                x: self.x.saturating_sub(1)..=self.x.saturating_add(self.width),
                y: self.y,
            },
            AdjacentRange {
                x: self.x.saturating_sub(1)..=self.x.saturating_add(self.width),
                y: self.y.saturating_add(1),
            },
        ]
        .iter()
        .filter(|r| r.y == gear.y && r.x.contains(&gear.x))
        .count()
            > 0
    }
}

struct Gear {
    gear: char,
    x: usize,
    y: usize,
}

fn parse(input: &str) -> (Vec<PartNumber>, Vec<Gear>) {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices().filter(|(_, c)| *c != '.').fold(
                (Vec::<PartNumber>::new(), Vec::<Gear>::new()),
                |mut acc, (x, c)| {
                    if !c.is_ascii_digit() {
                        acc.1.push(Gear { gear: c, x, y });
                        return acc;
                    }
                    if let Some(last_part) = acc.0.last_mut() {
                        if x == last_part.x + last_part.width {
                            last_part.width += 1;
                            last_part.number += &c.to_string();
                            return acc;
                        }
                    }
                    acc.0.push(PartNumber {
                        number: c.to_string(),
                        x,
                        y,
                        width: 1,
                    });
                    acc
                },
            )
        })
        .reduce(|mut acc, (mut part_numbers, mut gears)| {
            acc.0.append(&mut part_numbers);
            acc.1.append(&mut gears);
            acc
        })
        .unwrap()
}

fn part1(input: &str) -> u32 {
    let (part_numbers, gears) = parse(input);
    part_numbers
        .iter()
        .filter_map(|part_number| {
            let valid = gears
                .iter()
                .filter(|gear| part_number.is_adjacent(gear))
                .count()
                > 0;
            if valid {
                return Some(part_number.number.parse::<u32>().unwrap());
            }
            None
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let (part_numbers, gears) = parse(input);
    gears
        .iter()
        .filter_map(|gear| {
            if gear.gear == '*' {
                let adjacent_part_numbers = part_numbers
                    .iter()
                    .filter(|part_number| part_number.is_adjacent(gear))
                    .collect::<Vec<_>>();
                if adjacent_part_numbers.len() == 2 {
                    return Some(
                        adjacent_part_numbers
                            .iter()
                            .map(|part_number| part_number.number.parse::<u32>().unwrap())
                            .product::<u32>(),
                    );
                }
            }
            None
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 467835);
    }
}
