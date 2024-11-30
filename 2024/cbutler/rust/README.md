# AoC 2024 - rust edition

This is a Rust implementation of Advent of Code 2024.

## About

 - `main.rs` is the entry point to the program which allows you to run all puzzle solvers.

 - Puzzle solvers are implemented as functions inside rust modules. Each day is it's own sub module.

 - Just about every day's puzzle relies on the `data_file_reader` util.


## Getting Started

 - Copy the `boilerplate.rs` file and rename both the file and the functions inside to something like `dec01.rs`
<br />

 - Add a call to the solver in the `main.rs` file.

1. install dependencies

```sh

```

2. Run the project

```sh
cargo run -q
```


## Running the puzzle solvers

The interface for each puzzle looks like this: 

 ```cs
solvePart1(date: &str, use_test_data: bool)
solvePart2(date: &str, use_test_data: bool)
 ```

Rust does not support default values or optional parameters so all parameters must always be provided.
If you want to run a puzzle for a day other the the current day or use test data, you will need to pass the parameter in.

```rs
// run day XX part 1 using real data
boilerplate::solvePart1(test_date, false);

// run day XX part 2 using test data
boilerplate::solvePart2(test_date, true);
```

comment out any solvers you do not want to run.


## DataFileReader Interface and description

This Data File Reader is written for AoC and tightly coupled to the project structure. The constructor looks like this: 

```rs
pub fn data_file_reader(date: &str, use_test_data: bool, part: u8, verbose: bool) -> Result<Vec<String>, io::Error> {}
```

 - It assumes you are creating a data file each day with a specific name format: `dd-MM-yyyy-data.txt` for real data and `dd-MM-yyyy-partX-test-data.txt` for test data (where X is either 1 or 2.)
<br />

 - `date` - The date used to generate the filename in the format dd-MM-yyyy-data.txt.
<br />

 - `useTestData` - Indicates whether to use test data or real data (this is of course specific to AoC).
 <br />

 - `part` - The part of the puzzle (e.g., 1 or 2).
 <br />

 - `verbose` - Indicates whether to printout console logs or not.
 <br />
