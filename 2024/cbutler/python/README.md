# AoC 2024 - python edition

This is a Python implementation of Advent of Code 2024.

## About

 - `main.py` is the entry point to the program which allows you to run all the puzzle solvers.

 - Puzzle solvers are implemented as functions inside python modules. There is a module for each day.

 - Just about every day's puzzle relies on the `data_file_reader` util.

 - the `__init__.py` file in the puzzles folder makes that folder a package


## Getting started

 - Copy the `boilerplate.py` file and rename the file to something like `dec01.py`
<br />

 - Add a call to the solver in the `main.py` file.


1. Install dependencies:

```sh
pip install -r requirements.txt
```

2. Run the entry script:

```sh
python3 main.py
```


## Running the puzzle solvers

The function signature for each puzzle looks like this: 

 ```python
solve_part1(date: str, use_test_data: bool = False)
solve_part2(date: str, use_test_data: bool = False)
 ```

This interface provides sensible defaults to run the puzzle solver.
Python also support default values / optional parameters.
If you want to run a puzzle for a day other the the current day or use test data, you will need to pass the parameter in.

```python
# run day XX part 1 using real data
boilerplate.solve_part1(current_date)
	
# run day XX part 2 using test data
boilerplate.solve_part2(current_date, use_test_data=True)
```

comment out any solvers you do not want to run.


## DataFileReader Interface and description

This Data File Reader is written for AoC and tightly coupled to the project structure. The constructor looks like this: 

```python
def data_file_reader(filename: str = '', date: str = '', use_test_data: bool = False, part: int = 1, verbose: bool = False) -> list:
```

 - `filename` - The full path of the data file. If not provided, the filename will be generated from the date and the path assumed. It assumes you are creating a data file each day with a specific name format: `dd-MM-yyyy-data.txt` for real data and `dd-MM-yyyy-partX-test-data.txt` for test data (where X is either 1 or 2.)
<br />

 - `date` - The date used to generate the filename in the format dd-MM-yyyy-data.txt.
<br />

 - `useTestData` - Indicates whether to use test data or real data (this is of course specific to AoC).
 <br />

 - `part` - The part of the puzzle (e.g., 1 or 2).
 <br />

 - `verbose` - Indicates whether to printout console logs or not.
 <br />
