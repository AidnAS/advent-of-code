# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        24.274μs        32.056μs
02        21.862μs        25.714μs
03        34.686μs        26.530μs
04        36.320μs        44.492μs
05        10.022μs        63.779μs
06       175.000ns       490.000ns
07        82.454μs        88.713μs
08        82.366μs       432.739μs
09       161.481μs       156.603μs
10       177.357μs       810.756μs
-----------------------------------
Total:   2.313ms
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