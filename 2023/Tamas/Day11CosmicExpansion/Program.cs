using Day11CosmicExpansion;

var universe = Universe.LoadFrom("input.txt");
long distanceSum = universe.CalculateSumOfDistances(2);
Console.WriteLine(
    $"The sum of the distances of all galaxies is {distanceSum}.");
long distanceSumInOlderUniverse = universe.CalculateSumOfDistances(1_000_000);
Console.WriteLine(
    $"The sum of the distances of all galaxies given the correct age of the universe is {distanceSumInOlderUniverse}.");