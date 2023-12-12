# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        23.113μs        30.057μs
02        22.081μs        26.094μs
03        35.374μs        26.437μs
04        37.625μs        38.118μs
05        10.427μs        64.591μs
06       167.000ns       492.000ns
07        89.608μs        93.253μs
08        66.350μs       145.316μs
09       161.471μs       160.589μs
10        80.272μs        78.201μs
11        72.769μs        72.955μs
-----------------------------------
Total:   1.335ms
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