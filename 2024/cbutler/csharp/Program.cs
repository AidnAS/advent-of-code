using System;
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
var dec01Part1 = Dec01.solvePart1(currentDate, useTestData: false); // ans: 3508942
var dec01Part2 = Dec01.solvePart2(currentDate, useTestData: false); // ans: 26593248


// validate the solutions
if (runValidator) 
{
	Console.WriteLine($"{PuzzleValidator.Validate(3508942, dec01Part1, "Dec 01, Part 1")}");
	Console.WriteLine($"{PuzzleValidator.Validate(26593248, dec01Part2, "Dec 01, Part 2")}");
	Console.WriteLine("-----------------\n");
}
