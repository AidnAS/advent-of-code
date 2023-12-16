# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        23.762μs        30.763μs
02        21.559μs        25.643μs
03        35.327μs        25.084μs
04        36.515μs        38.097μs
05        10.106μs        63.714μs
06       158.000ns       483.000ns
07        88.700μs        91.449μs
08        65.141μs       158.992μs
09       162.936μs       162.603μs
10        84.447μs        85.696μs
11        71.787μs        71.941μs
12       303.892μs       691.270μs
13        22.885μs        23.726μs
14        16.271μs         7.146ms
15        28.566μs        91.245μs
-----------------------------------
Total:   9.678ms
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