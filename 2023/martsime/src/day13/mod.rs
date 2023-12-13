use crate::solution::Solution;

struct Image<'a> {
    bytes: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Image<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        let width = bytes.iter().position(|&b| b == b'\n').unwrap() + 1;
        let height = bytes.len() / width;

        Self {
            bytes,
            width,
            height,
        }
    }

    #[inline]
    fn process(&self, smudges: usize) -> u64 {
        for y in 0..self.height - 1 {
            let mut y1 = y;
            let mut y2 = y + 1;
            let mut differences = 0;

            while differences <= smudges {
                differences += (0..self.width - 1).fold(0, |acc, x| {
                    acc + (self.bytes[y1 * self.width + x] != self.bytes[y2 * self.width + x])
                        as usize
                });
                if y1 == 0 || y2 == self.height - 1 {
                    break;
                }
                y1 -= 1;
                y2 += 1;
            }

            if differences == smudges {
                return ((y + 1) * 100) as u64;
            }
        }
        for x in 0..self.width - 2 {
            let mut x1 = x;
            let mut x2 = x + 1;
            let mut differences = 0;

            while differences <= smudges {
                differences += (0..self.height).fold(0, |acc, y| {
                    acc + (self.bytes[y * self.width + x1] != self.bytes[y * self.width + x2])
                        as usize
                });
                if x1 == 0 || x2 == self.width - 2 {
                    break;
                }
                x1 -= 1;
                x2 += 1;
            }

            if differences == smudges {
                return (x + 1) as u64;
            }
        }
        0
    }
}

fn parse_images(mut input: &[u8]) -> Vec<Image> {
    let mut images: Vec<Image> = Vec::with_capacity(1000);
    let mut index = 0;
    while index < input.len() - 1 {
        if input[index..=index + 1] == [b'\n', b'\n'] {
            images.push(Image::new(&input[..=index]));
            input = &input[index + 2..];
            index = 0;
            continue;
        }
        index += 1;
    }
    images.push(Image::new(input));
    images
}

fn solve_part01(input: &[u8]) -> u64 {
    parse_images(input)
        .iter()
        .map(|image| image.process(0))
        .sum()
}

fn solve_part02(input: &[u8]) -> u64 {
    parse_images(input)
        .iter()
        .map(|image| image.process(1))
        .sum()
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
    fn day13_part01() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part01(input), 405);
    }

    #[test]
    fn day13_part02() {
        let input = include_bytes!("input_test.txt");
        assert_eq!(solve_part02(input), 400);
    }
}
