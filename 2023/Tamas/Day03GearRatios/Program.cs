using Day03GearRatios;

var schematic = Schematic.ReadFrom("input.txt");
var sumPartNumbers = schematic.FindParts()
    .Sum(part => part.Value);
Console.WriteLine($"The sum of all part numbers is {sumPartNumbers}.");

var sumGearRatios = schematic.FindGears()
    .Sum(gear => gear.Ratio);
Console.WriteLine($"The sum of all gear ratios is {sumGearRatios}.");