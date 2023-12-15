fn part1(input: &str) -> usize {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|step| {
            step.chars()
                .fold(0, |acc, ch| ((acc + (ch as usize)) * 17) % 256)
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|step| {
            step.chars().fold(
                (0, "".to_string(), ' ', 0),
                |(mut box_nr, mut label, mut operation, mut value), ch| {
                    match ch {
                        c if c.is_ascii_alphabetic() => {
                            box_nr = ((box_nr + (c as usize)) * 17) % 256;
                            label.push(c);
                        }
                        c if c == '-' || c == '=' => {
                            operation = c;
                        }
                        c if c.is_ascii_digit() => {
                            value = c.to_digit(10).unwrap() as usize;
                        }
                        _ => {}
                    }
                    (box_nr, label, operation, value)
                },
            )
        })
        .fold(
            vec![Vec::<(String, usize)>::new(); 256],
            |mut boxes, (box_nr, label, operation, value)| {
                let the_box = boxes.get_mut(box_nr).unwrap();
                if let Some(i) = the_box.iter().position(|(l, _)| label == *l) {
                    if operation == '=' {
                        the_box[i] = (label.to_string(), value);
                    } else {
                        the_box.remove(i);
                    }
                } else if operation == '=' {
                    the_box.push((label.to_string(), value));
                }
                boxes
            },
        )
        .iter()
        .enumerate()
        .map(|(i, the_box)| {
            the_box
                .iter()
                .enumerate()
                .map(|(j, lens)| (i + 1) * (j + 1) * lens.1)
                .sum::<usize>()
        })
        .sum::<usize>()
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
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 145);
    }
}
