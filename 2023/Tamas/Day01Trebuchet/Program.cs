using Day01Trebuchet;

var allLines = File.ReadAllLines("input.txt");
var parser = Parser.LoadRulesFrom("rules1.txt");
var sum = allLines.Sum(parser.GetNumberFromFirstAndLastDigit);
Console.WriteLine($"The sum of the numbers consisting of the first and last digits in every line is {sum}.");

parser = Parser.LoadRulesFrom("rules2.txt");
sum = allLines.Sum(parser.GetNumberFromFirstAndLastDigit);
Console.WriteLine($"The sum of the numbers consisting of the real first and last digits in every line is {sum}.");
