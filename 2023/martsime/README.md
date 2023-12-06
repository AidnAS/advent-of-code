# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        19.832μs        30.607μs
02        23.088μs        28.322μs
03        36.416μs        26.174μs
04        38.063μs        39.007μs
05        10.449μs        65.899μs
06       166.000ns       507.000ns
-----------------------------------
Total: 318.530μs
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