# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        23.143μs        31.503μs
02        21.719μs        25.912μs
03        34.870μs        24.829μs
04        37.174μs        37.946μs
05        10.090μs        64.034μs
06       157.000ns       482.000ns
07        83.830μs        86.878μs
08        82.302μs       430.257μs
09       156.308μs       156.178μs
10       182.761μs       812.275μs
11        73.044μs        72.596μs
-----------------------------------
Total:   2.448ms
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