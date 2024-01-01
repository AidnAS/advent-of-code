using Day18LavaductLagoon;

var digPlan = DigPlan.Load("input.txt");
long volume = digPlan.CalculateVolume(usingColor: false);
Console.WriteLine($"The volume of the hole is {volume}.");

volume = digPlan.CalculateVolume(usingColor: true);
Console.WriteLine($"The volume of the true hole is {volume}.");
