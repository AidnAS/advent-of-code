using System.Diagnostics;
using Aoc2024;

var stopwatch = Stopwatch.StartNew();
var res = Problem8.Part2();
stopwatch.Stop();

Console.WriteLine($"Took: {stopwatch.ElapsedMilliseconds}ms");
Console.WriteLine(res);