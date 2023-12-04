# Advent of Code 2023

My attempt at [AoC 2023](https://adventofcode.com/2023) using Python.

## How to run

Requires Python 3.11 to run. In theory, it should be as simple as cloning the repo, `cd` to repo
root and `poetry install`.

From there you should be able to do `poetry run all` and the implemented solvers for each day will run. If you want to
run a single day, do e.g. `poetry run one`, `poetry run fifteen`, etc.

For certain days there may exist a `golf.py`. This makes use of `open(0)` to read from stdin, and must be run as
`python aoc/{day}/golf.py < aoc/{day}/input.txt`.

### Tests

Tests are written using `pytest`. To run them, simply run `pytest` from repository root, or `poetry run tests`.
