using Day15LensLibrary;

var input = File.ReadAllText("input.txt").Trim();
var instructions = input.Split(',');
int sum = instructions.Sum(HashAlgorithm.Run);
Console.WriteLine($"The sum of the results from running the HASH algorithm on every step is {sum}.");

var facility = new Facility();
facility.Execute(instructions);
sum = facility.GetCombinedFocusingPower();
Console.WriteLine($"The combined focusing power of the resulting configuration is {sum}.");
