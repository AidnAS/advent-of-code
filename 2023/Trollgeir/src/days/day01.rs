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

pub fn find_first_and_last_number(content: String, number_literals: &[&str]) -> u32 {
    // Further optimization could be done by terminating literal searches early if we've iterated past the first known literal char position
    let first_number_char = number_literals
        .iter()
        .filter_map(|literal| content.find(literal).map(|position| (*literal, position)))
        .min_by_key(|(_, position)| *position)
        .map(|(literal, _)| number_literal_to_char(literal))
        .expect("At least one number literal exists");

    let last_number_char = number_literals
        .iter()
        .filter_map(|literal| content.rfind(literal).map(|position| (*literal, position)))
        .max_by_key(|(_, position)| *position)
        .map(|(literal, _)| number_literal_to_char(literal))
        .expect("At least one number literal exists");

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
            find_first_and_last_number(content, &number_literals)
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
            find_first_and_last_number(content, &number_literals)
        })
        .sum();

    (Solution::U32(sum_numbers_1), Solution::U32(sum_numbers_2))
}
