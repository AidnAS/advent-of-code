using Day16TheFloorWillBeLava;

var contraption = Contraption.Load("input.txt");
int count = contraption.CountEnergizedTilesRightFromTopLeftCorner();
Console.WriteLine($"The number of resulting energized tiles is {count}.");

count = contraption.GetMaxEnergizedTiles();
Console.WriteLine($"The maximum number of energized tiles is {count}.");