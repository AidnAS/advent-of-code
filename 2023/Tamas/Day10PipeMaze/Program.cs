using Day10PipeMaze;

var map = PipeMap.LoadFrom("input.txt");
int distance = map.GetDistanceToFarthestLoopPoint();
Console.WriteLine($"The farthest pipe section is {distance} steps away.");

int area = map.CalculateLoopInsideArea();
Console.WriteLine($"The inside of the loop counts {area} tiles.");