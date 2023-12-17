# 2023 Advent of Code 🦀

This year I have chosen to solve the puzzles in rust once again, however new from this year is that the code is optimized purefully for performance.
If any humans stuble upon it, don't expect it to be very readable!

## Performance
```
Day        Part 1          Part 2
-----------------------------------
01        24.303μs        31.533μs
02        22.088μs        25.968μs
03        35.257μs        24.709μs
04        43.847μs        38.920μs
05        10.448μs        66.167μs
06       182.000ns       511.000ns
07        86.965μs        91.205μs
08        67.112μs       148.940μs
09       162.609μs       159.103μs
10        80.403μs        80.769μs
11        72.523μs        72.258μs
12       305.543μs       685.037μs
13        22.973μs        23.776μs
14        32.875μs         8.319ms
15        28.486μs        92.848μs
16        45.759μs         1.521ms
17       169.936ms       548.741ms
-----------------------------------
Total: 731.101ms
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