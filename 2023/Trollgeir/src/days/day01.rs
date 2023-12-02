use crate::{Solution, SolutionPair};

fn number_literal_to_char(number: &str) -> char {
    match number {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => number.chars().next().expect("String has content"),
    }
}

pub fn find_first_and_last_number(content: &str, number_literals: &[&str]) -> u32 {
    // Shrink the relevant sub-content we're searching in when we find improvements
    let mut sub_content: &str = content;
    let mut first_number_literal = None;

    for number_literal in number_literals {
        if let Some(position) = sub_content.find(number_literal) {
            first_number_literal = Some(number_literal);
            sub_content = &sub_content[..position + 1];
        }
    }

    let first_number_char = number_literal_to_char(
        first_number_literal.expect("At least one number literal was found"),
    );

    let mut sub_content: &str = content;
    let mut last_number_literal = None;

    for number_literal in number_literals {
        if let Some(position) = sub_content.rfind(number_literal) {
            last_number_literal = Some(number_literal);
            sub_content = &sub_content[position..];
        }
    }

    let last_number_char =
        number_literal_to_char(last_number_literal.expect("At least one number literal was found"));

    format!("{}{}", first_number_char, last_number_char)
        .parse::<u32>()
        .expect("Number can be parsed")
}

pub fn solve() -> SolutionPair {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let number_literals = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let sum_numbers_1: u32 = reader
        .lines()
        .map(|line| -> u32 {
            let content = line.expect("Line can be parsed");
            find_first_and_last_number(&content, &number_literals)
        })
        .sum();

    let reader = BufReader::new(File::open("input/input.txt").expect("Can read file"));
    let number_literals = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let sum_numbers_2: u32 = reader
        .lines()
        .map(|line| -> u32 {
            let content = line.expect("Line can be parsed");
            find_first_and_last_number(&content, &number_literals)
        })
        .sum();

    (Solution::U32(sum_numbers_1), Solution::U32(sum_numbers_2))
}
