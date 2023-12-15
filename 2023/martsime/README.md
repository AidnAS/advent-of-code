# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        23.705μs        29.331μs
02        22.328μs        26.439μs
03        34.543μs        24.540μs
04        36.863μs        37.413μs
05        10.123μs        62.984μs
06       176.000ns       482.000ns
07        84.476μs        89.825μs
08        65.132μs       172.575μs
09       159.653μs       159.809μs
10        82.121μs        83.195μs
11        71.241μs        71.064μs
12       302.187μs       679.612μs
13        22.872μs        23.665μs
-----------------------------------
Total:   2.376ms
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