# 2023 Advent of Code (Python)

This repository implements solutions to the puzzles in the
[2023 Advent of Code](https://adventofcode.com/2023) using Python.


## Preface

This was a vehicle to refresh my knowledge of Python, so I presume
not everything done here will be deemed idiomatic by language specialists.

Generally speaking, the solutions are organised predominantly for comprehension.
They strive to arrive at an answer in a reasonable period of time, but they
typically prioritise optimal understanding over optimal performance.

The examples are representative of my thinking and coding style.


## Getting Started

### Prerequisites

The project requires `python 3.12.0`, but any reasonably current version of
Python will likely work. I tend to code done the middle of any language
specification.

If you use a Python manager that responds to `.tool-versions`, you should
be switched to correct version automatically. I recommend [ASDF](https://github.com/asdf-vm/asdf)
for those on platforms that support it.

### Installation

Python's package manager should be included in the installation, but it is often
slightly out of date.  Let's ensure we have the latest and greatest.

```bash
$ pip install --upgrade pip
```

Next we need to install and create a virtual environment for the project. This will
create a clean space for managing our python dependencies. This library should
also be included automatically in your Python installation.

```bash
$ python -m venv venv
```

Now, load your virtual environment.

```bash
source venv/bin/activate
```

Finally, install the project dependencies.

```bash
$ pip install --upgrade pip
$ pip install -r requirements.txt
```

### File Structure

- [data](./data):   Puzzle input organised by day
- [site](./site):   A local version of the instruction pages
- [src](./src):     Puzzle solutions organised by day
- [test](./test):   A simple set of regression tests


### Running Daily Solutions

Modify the code in `main.py` to run the solutions for the appropriate
day. Then, simply execute the following command in your terminal from the
project root.

```
$ make run
```


### Running Tests

The only tests are a set of checks to verify solved puzzles.

I often refactor my solutions for clarity (or as I learn new
techniques in subsequent puzzles), so it is helpful to have
these simple tests to give my refactors some confidence.

To execute the tests, simply execute the following command in
your terminal from the project root.

```
$ make verify
```