use crate::solution::Solution;
use crate::utils::parse_number_from_bytes;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

#[inline(always)]
fn is_next_to_symbol(width: i32, height: i32, input: &[u8], positions: &[(i32, i32)]) -> bool {
    for &(y, x) in positions.iter() {
        for &(dy, dx) in DIRECTIONS.iter() {
            let new_x = x + dx;
            let new_y = y + dy;
            if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                let index = (new_y * (width + 1) + new_x) as usize;
                let byte = input[index];
                if byte.is_ascii_digit() || byte == b'.' {
                    continue;
                } else {
                    return true;
                }
            }
        }
    }
    false
}

#[inline(always)]
fn extract_number(
    width: i32,
    input: &[u8],
    position: (i32, i32),
    used_positions: &mut [(i32, i32)],
) -> u32 {
    let (row, col) = position;
    for i in -2..=0 {
        let (y, x) = (row, col + i);
        let start_index = (y * (width + 1) + x) as usize;
        if x >= 0 && x < width {
            let bytes = &input[start_index..start_index + 3];
            match (
                bytes[0].is_ascii_digit(),
                bytes[1].is_ascii_digit(),
                bytes[2].is_ascii_digit(),
            ) {
                (true, true, true) => {
                    used_positions[0] = (y, x);
                    used_positions[1] = (y, x + 1);
                    used_positions[2] = (y, x + 2);
                    return parse_number_from_bytes::<u32>(&bytes[0..3]);
                }
                (true, true, false) => {
                    used_positions[0] = (y, x);
                    used_positions[1] = (y, x + 1);
                    return parse_number_from_bytes::<u32>(&bytes[0..2]);
                }
                (true, false, false) => {
                    used_positions[0] = (y, x);
                    return parse_number_from_bytes::<u32>(&bytes[0..1]);
                }
                _ => {}
            }
        }
    }

    unreachable!();
}

pub fn solve_part01(input: &[u8]) -> u32 {
    let width = input.iter().position(|b| *b == b'\n').unwrap();
    // Subtract the bytes from all the new line characters except the last one
    let height = (input.len() - width + 1) / width;
    const BUFFER_SIZE: usize = 3;

    let mut total = 0;

    let mut number_buffer = [0u8; BUFFER_SIZE];
    let mut position_buffer: [(i32, i32); BUFFER_SIZE] = [(0, 0); BUFFER_SIZE];
    let mut buffer_index: usize = 0;

    for row in 0..height {
        if buffer_index > 0 {
            if is_next_to_symbol(
                width as i32,
                height as i32,
                input,
                &position_buffer[0..buffer_index],
            ) {
                let number = parse_number_from_bytes::<u32>(&number_buffer[0..buffer_index]);
                total += number;
            }
            buffer_index = 0;
        }
        for column in 0..width {
            let byte = input[row * (width + 1) + column];
            if byte.is_ascii_digit() {
                number_buffer[buffer_index] = byte;
                position_buffer[buffer_index] = (row as i32, column as i32);
                buffer_index += 1;
            } else if buffer_index > 0 {
                if is_next_to_symbol(
                    width as i32,
                    height as i32,
                    input,
                    &position_buffer[0..buffer_index],
                ) {
                    let number = parse_number_from_bytes::<u32>(&number_buffer[0..buffer_index]);

                    total += number;
                }
                buffer_index = 0;
            }
        }
    }

    total
}

pub fn solve_part02(input: &[u8]) -> u32 {
    let width = input.iter().position(|b| *b == b'\n').unwrap();
    // Subtract the bytes from all the new line characters except the last one
    let height = (input.len() - width + 1) / width;
    let mut total = 0;

    const NUMBER_BUFFER_SIZE: usize = 2;
    const POSITION_BUFFER_SIZE: usize = NUMBER_BUFFER_SIZE * 3;

    let mut number_buffer = [0u32; NUMBER_BUFFER_SIZE];
    let mut number_buffer_index: usize = 0;
    let mut position_buffer: [(i32, i32); POSITION_BUFFER_SIZE * 3] =
        [(0, 0); POSITION_BUFFER_SIZE * 3];
    let mut position_buffer_index: usize = 0;

    for row in 0..height {
        for column in 0..width {
            let byte = input[row * (width + 1) + column];
            if byte == b'*' {
                for (dy, dx) in DIRECTIONS.iter() {
                    let (y, x) = (row as i32 + dy, column as i32 + dx);
                    if y >= 0 && y < height as i32 && x >= 0 && x < width as i32 {
                        // Check if the position has already been used
                        if position_buffer[0..position_buffer_index]
                            .iter()
                            .any(|position| position == &(y, x))
                        {
                            continue;
                        }
                        let byte = &input[y as usize * (width + 1) + x as usize];
                        if byte.is_ascii_digit() {
                            // Check if we have already found two numbers, then we can stop
                            if number_buffer_index == NUMBER_BUFFER_SIZE {
                                number_buffer_index += 1;
                                break;
                            }
                            let number = extract_number(
                                width as i32,
                                input,
                                (y, x),
                                &mut position_buffer[position_buffer_index..],
                            );

                            if number < 10 {
                                position_buffer_index += 1;
                            } else if number < 100 {
                                position_buffer_index += 2;
                            } else {
                                position_buffer_index += 3;
                            }

                            number_buffer[number_buffer_index] = number;
                            number_buffer_index += 1;
                        }
                    }
                }
                if number_buffer_index == NUMBER_BUFFER_SIZE {
                    total += number_buffer[0] * number_buffer[1];
                }
                number_buffer_index = 0;
                position_buffer_index = 0;
            }
        }
    }
    total
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
    fn day03_part01() {
        let input = include_bytes!("input_test.txt");
        let solution = solve_part01(input);
        assert_eq!(solution, 4361);
    }

    #[test]
    fn day03_part02() {
        let input = include_bytes!("input_test.txt");
        let solution = solve_part02(input);
        assert_eq!(solution, 467835);
    }
}
