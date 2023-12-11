# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        22.583μs        32.178μs
02        21.922μs        26.147μs
03        35.298μs        25.003μs
04        37.059μs        37.912μs
05        10.008μs        64.242μs
06       175.000ns       481.000ns
07        81.416μs        87.574μs
08        82.558μs       430.670μs
09       158.876μs       157.099μs
10        90.645μs        92.715μs
11        71.896μs        72.081μs
-----------------------------------
Total:   1.639ms
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