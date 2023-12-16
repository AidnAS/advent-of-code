# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        23.836μs        30.626μs
02        21.768μs        25.351μs
03        36.026μs        25.300μs
04        36.623μs        37.973μs
05        10.087μs        63.803μs
06       158.000ns       486.000ns
07        84.839μs        92.451μs
08        65.132μs       143.004μs
09       164.142μs       164.496μs
10        85.455μs        87.100μs
11        72.429μs        72.356μs
12       303.843μs       684.332μs
13        22.907μs        23.762μs
14        15.781μs         7.138ms
-----------------------------------
Total:   9.532ms
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