use regex::Regex;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // read the file input.txt
    let file = std::fs::File::open("input.txt").unwrap();

    let mut sum = 0;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Can't read line");
        let iter = re.find_iter(&line);
        for mat in iter {
            //println!("{:?}", mat.as_str());
            sum += mul(mat.as_str());
        }
    }
    println!("sum: {}", sum);
}

fn mul(m: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures(m)
        .map(|cap| {
            let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            a * b
        })
        .unwrap_or(0)
}
