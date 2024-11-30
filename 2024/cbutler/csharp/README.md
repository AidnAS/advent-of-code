# AoC 2024 - C# edition

This is a C# implementation of Advent of Code 2024.

## About 

 - `Program.cs` is the entry point to the program which allows you to run all puzzle solvers.

 - Puzzle solvers are implemented as csharp static classes (so you do not need to instantiate them)

 - Just about every day's puzzle relies on the `DataFileReader` util.

## Getting Started

 - Copy the `Boilerplate.cs` file and rename both the file and class it to something like `Dec01.cs`
<br />

 - Add a call to the solver in the `Program.cs` file.

To run the project:

```
dotnet run
```

To clean: 

```
dotnet clean
```


## Running the puzzle solvers

The interface for each puzzle looks like this: 

 ```cs
public interface IPuzzle
{
	public static void solvePart1(string? date, bool useTestData = false);
	public static void solvePart2(string? date, bool useTestData = false);
}
 ```

This interface provides sensible defaults to run the puzzle solver. 
If you want to run a puzzle for a day other the the current day or use test data, you will need to pass the parameter in.

```cs
// run day XX part 1 using real data
DecXX.solvePart1(currentDate);

// run day XX part 2 using test data
DecXX.solvePart2(currentDate, useTestData = true);
```

comment out any solvers you do not want to run.



## DataFileReader Interface and description

This Data File Reader is written for AoC and tightly coupled to the project structure. The constructor looks like this: 

```cs
public DataFileReader(
	string? filename = null,
	string? date = null,
	bool? useTestData = false,
	int? part = 0,
	bool verbose = false
)
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
