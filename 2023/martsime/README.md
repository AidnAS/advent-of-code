# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        23.602μs        28.850μs
02        22.146μs        25.959μs
03        35.074μs        24.679μs
04        35.979μs        38.294μs
05        10.153μs        63.259μs
06       153.000ns       486.000ns
07        85.179μs        88.426μs
08        65.258μs       169.690μs
09       160.255μs       160.047μs
10        86.964μs        85.897μs
11        74.242μs        72.737μs
12       302.696μs       678.488μs
13        23.134μs        23.999μs
14        16.101μs         7.249ms
15        27.200μs        90.136μs
16        43.690μs         1.474ms
17         5.553ms         8.469ms
-----------------------------------
Total:  25.307ms
```

## Requirements

To run the code you need a version of rust installed, preferably the newest, which is 1.74 at time of writing.

## Run the code

Run all puzzles already solved
```shell
cargo run --release
```

Run a specific puzzle
```shell
cargo run --release <day> <part>
```

For example
```shell
cargo run --release 1 1
```

## Tests
Test the solutions against the sample inputs given in the puzzle description
```shell
cargo test 
```

## Benchmarks

Benchmark the solutions using [criterion](https://github.com/bheisler/criterion.rs).
```shell
cargo bench
```