using Day14ParabolicReflectorDish;

string inputFile = "input.txt";
var platform = Platform.Load(inputFile);
platform.Tilt(Direction.North);
int load = platform.GetLoadNorth();
Console.WriteLine($"The total load on the north support beams after tilting north is {load}.");

platform = Platform.Load(inputFile);
platform.PerformManyCycles(1000000000);
load = platform.GetLoadNorth();
Console.WriteLine($"The total load on the north support beams after many cycles is {load}.");
