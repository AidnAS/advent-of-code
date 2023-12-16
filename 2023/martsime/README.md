# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        22.759μs        31.394μs
02        21.538μs        25.881μs
03        35.210μs        24.850μs
04        36.691μs        37.822μs
05        10.065μs        63.349μs
06       171.000ns       487.000ns
07        81.431μs        87.990μs
08        65.145μs       141.898μs
09       162.369μs       162.355μs
10        84.552μs        82.947μs
11        71.822μs        71.813μs
12       302.146μs       679.520μs
13        23.109μs        24.049μs
14        16.089μs         7.060ms
15        27.439μs        93.252μs
16        44.226μs         1.497ms
-----------------------------------
Total:  11.090ms
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