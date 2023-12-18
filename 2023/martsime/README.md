# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        20.002μs        32.210μs
02        21.757μs        25.782μs
03        33.819μs        24.751μs
04        36.644μs        37.800μs
05        10.104μs        64.151μs
06       169.000ns       483.000ns
07        83.811μs        85.929μs
08        65.140μs       157.349μs
09       157.293μs       157.968μs
10        85.719μs        82.836μs
11        71.816μs        71.702μs
12       303.301μs       682.961μs
13        22.887μs        23.639μs
14        15.947μs         7.658ms
15        28.218μs        93.308μs
16        44.836μs         1.504ms
17         5.622ms         8.733ms
18         9.036μs        17.916μs
-----------------------------------
Total:  26.086ms
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