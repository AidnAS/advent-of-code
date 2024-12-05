
using csharp.Puzzles;
using csharp.utils;


var currentDate = DateTime.Now.ToString("dd-MM-yyyy");
var runValidator = true;

Console.WriteLine($"AoC: {currentDate}");
Console.WriteLine("-----------------\n");

// test boilerplate
// var testDate = "01-12-2000";
// Boilerplate.solvePart1(testDate, useTestData: true);
// Boilerplate.solvePart2(testDate);

// run puzzle solvers
// var dec01Part1 = Dec01.solvePart1(currentDate, useTestData: false); // ans: 3508942
// var dec01Part2 = Dec01.solvePart2(currentDate, useTestData: false); // ans: 26593248

// var dec02Part1 = Dec02.solvePart1("02-12-2024", useTestData: false); // ans: 282
// var dec02Part2 = Dec02.solvePart2("02-12-2024", useTestData: false); // ans: 349

// var dec03Part1 = Dec03.solvePart1("03-12-2024", useTestData: false); // ans: 188192787
// var dec03Part2 = Dec03.solvePart2("03-12-2024", useTestData: false); // ans: 113965544

// var dec04Part1 = Dec04.solvePart1(currentDate, useTestData: false); // ans: 2464
// var dec04Part2 = Dec04.solvePart2(currentDate, useTestData: false); // ans: 1982

var dec05Part1 = Dec05.solvePart1(currentDate, useTestData: true); // ans: 
// var dec05Part2 = Dec05.solvePart2(currentDate, useTestData: true); // ans: 

// validate the solutions
if (runValidator) 
{
	// Console.WriteLine($"{PuzzleValidator.Validate(3508942, dec01Part1, "Dec 01, Part 1")}");
	// Console.WriteLine($"{PuzzleValidator.Validate(26593248, dec01Part2, "Dec 01, Part 2")}");
	// Console.WriteLine("-----------------\n");

	//  Console.WriteLine($"{PuzzleValidator.Validate(282, dec02Part1, "Dec 02, Part 1")}");
	//  Console.WriteLine($"{PuzzleValidator.Validate(349, dec02Part2, "Dec 02, Part 2")}");
	//  Console.WriteLine("-----------------\n");

	// Console.WriteLine($"{PuzzleValidator.Validate(188192787, dec03Part1, "Dec 03, Part 1")}");
	// Console.WriteLine($"{PuzzleValidator.Validate(113965544, dec03Part2, "Dec 03, Part 2")}");
	// Console.WriteLine("-----------------\n");

	// Console.WriteLine($"{PuzzleValidator.Validate(2464, dec04Part1, "Dec 04, Part 1")}");
	// Console.WriteLine($"{PuzzleValidator.Validate(1982, dec04Part2, "Dec 04, Part 2")}");
	// Console.WriteLine("-----------------\n");

}
