using Day08HauntedWasteland;

var map = Map.LoadFrom("input.txt");
int stepCount = map.CountStepsToDestination();
Console.WriteLine($"The number of steps to destination is {stepCount}.");

long ghostStepCount = map.CountStepsToGhostDestinations();
Console.WriteLine($"It takes {ghostStepCount} steps for all ghosts to simultaneously arrive at a destination.");