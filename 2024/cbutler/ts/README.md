# AoC 2024 - TypeScript edition

This is a TypeScript implementation of Advent of Code 2024.

## About 

 - `./src/index.ts` is the entry point to the program which allows you to run all puzzle solvers.

 - Puzzle solvers are implemented as functions.

 - Just about every day's puzzle relies on the `dataFileReader` util.

 - This project uses:
   - `TypeScript` as the programming language
	 - `esbuild` as the typescript compiler
	 - `node` and `npm` to make using npm packages easier
	 - `concurrently` to run multiple scripts simultaneously
	 - `nodemon` to automatically re-run the project when file changes are detected


## Getting Started

 - Copy the `boilerplate.ts` file and rename both the file and the functions inside to something like `dec01.ts`
<br />

 - Add a call to the solver in the `index.ts` file.

1. If you do not have node installed follow [these steps](https://nodejs.org/en/download/package-manager) to install.

2. To install the typescript compiler:

```
npm install -g typescript ts-node
```

verify with:

```
tsc --help
```

3. To run the project:

```
npm run start
```


## Running the puzzle solvers

The interface for each puzzle looks like this: 

 ```ts
type PuzzleSolver = (date?: string, useTestData?: boolean) => void;
 ```

This interface provides sensible defaults to run the puzzle solver. 
If you want to run a puzzle for a day other the the current day or use test data, you will need to pass the parameter in.

```ts
// run day XX part 1 using real data
boilerplate.solvePart1(currentDate, true);

// run day XX part 2 using test data
boilerplate.solvePart2(currentDate);
```

comment out any solvers you do not want to run.


## DataFileReader Interface and description

This Data File Reader is written for AoC and tightly coupled to the project structure. The constructor looks like this: 

```ts
interface IFileReader {
	filename?: string;
	date?: string;
	useTestData?: boolean;
	part?: number;
	verbose?: boolean;
}
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
