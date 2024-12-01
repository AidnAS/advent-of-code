use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // read in 2 lists from a file
    let (list1, list2) = read_list("input.txt");

    println!("length of list1: {}", list1.len());
    println!("length of list2: {}", list2.len());

    let mut distance = 0;

    for i in 0..list1.len() {
        distance = distance + (list1[i] - list2[i]).abs();
    }

    println!("The distance between the two lists is: {}", distance);
}

fn read_list(filename: &str) -> (Vec<i32>, Vec<i32>) {
    // read in the file
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    // read in the lines
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // each line has 2 numbers separated by space
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().trim().parse().expect("Not a number"));
        list2.push(iter.next().unwrap().trim().parse().expect("Not a number"));
    }

    list1.sort();
    list2.sort();

    // sort the lists
    (list1, list2)
}
