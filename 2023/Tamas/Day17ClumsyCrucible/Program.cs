using System.Diagnostics;
using Day17ClumsyCrucible;

var city = City.Load("input.txt");
int heatLoss = city.CalculateMinHeatLoss(
    minDistanceInOneDirection: 1, 
    maxDistanceInOneDiretion: 3);
Console.WriteLine($"The heat loss of the best path is {heatLoss}.");

heatLoss = city.CalculateMinHeatLoss(
    minDistanceInOneDirection: 4,
    maxDistanceInOneDiretion: 10);
Console.WriteLine($"The heat loss of the best path with an ultra crucible is {heatLoss}.");
