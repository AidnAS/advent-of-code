use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let (list1, list2) = read_list("input.txt");

    let mut sum = 0;

    for i in 0..list1.len() {
        let occ = count_occurrences(&list2, list1[i]);
        println!("Number: {}, Occurrences: {}", list1[i], occ);
        sum += list1[i] * occ;
    }

    println!("Sum: {}", sum);
}

fn read_list(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        // split the line and insert into 2 lists
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().parse::<i32>().unwrap());
        list2.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}

// write a function that counts the number of occurenses of a number in a list
fn count_occurrences(list: &Vec<i32>, number: i32) -> i32 {
    let mut count = 0;

    for i in 0..list.len() {
        // early exit condition
        if list[i] > number {
            break;
        }
        if list[i] == number {
            count += 1;
        }
    }

    count
}
