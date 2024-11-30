using System;
using csharp.Puzzles;

var currentDate = DateTime.Now.ToString("dd-MM-yyyy");

Console.WriteLine($"AoC: {currentDate}");
Console.WriteLine("-----------------\n");

// test boilerplate
var testDate = "01-12-2000";
Boilerplate.solvePart1(testDate, useTestData: true);
Boilerplate.solvePart2(testDate);

// run puzzle solvers
// Dec01.solvePart1(currentDate);
// Dec01.solvePart2(currentDate);