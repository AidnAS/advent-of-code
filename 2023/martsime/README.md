# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        23.833μs        29.362μs
02        21.898μs        25.894μs
03        35.318μs        26.285μs
04        37.298μs        37.781μs
05        10.109μs        65.239μs
06       178.000ns       497.000ns
07        83.134μs        92.105μs
08        65.645μs       168.849μs
09       160.527μs       159.674μs
10        82.151μs        83.543μs
11        72.819μs        73.076μs
12       304.947μs       685.244μs
13        23.163μs        23.987μs
14        16.439μs         7.606ms
15        27.536μs        93.811μs
16       929.036μs       136.685ms
-----------------------------------
Total: 147.750ms
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