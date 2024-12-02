use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // read from a file
    let file = File::open("input.txt").expect("Can't open file");

    let mut safe_count = 0;

    // loop every line
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Can't read line");
        let mut iter = line.split_whitespace();
        let mut levels = Vec::<i32>::new();

        // push until iter is empty
        while let Some(s) = iter.next() {
            levels.push(s.trim().parse().expect("Not a number"));
        }

        if is_safe(&levels) && (is_asc(&levels) || is_desc(&levels)) {
            safe_count += 1;
            //println!("{:?} Safe", levels);
        }
    }

    println!("Safe count: {}", safe_count);
}

fn is_safe(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() - 1 {
        let abs = (levels[i] - levels[i + 1]).abs();
        if abs < 1 || abs > 3 {
            return false;
        }
    }
    true
}

fn is_asc(levels: &Vec<i32>) -> bool {
    let mut last_value = levels[0];
    for i in 1..levels.len() {
        if last_value > levels[i] {
            return false;
        }
        last_value = levels[i];
    }
    true
}

fn is_desc(levels: &Vec<i32>) -> bool {
    let mut last_value = levels[0];
    for i in 1..levels.len() {
        if last_value < levels[i] {
            return false;
        }
        last_value = levels[i];
    }
    true
}
