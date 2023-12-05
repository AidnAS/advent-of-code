using Day05IfYouGiveASeedAFertilizer;

var almanac = Almanac.ParseFrom("input.txt");
uint closestLocation = almanac.FindClosestLocation();
Console.WriteLine($"The closest location is {closestLocation}.");
uint properClosestLocation = almanac.FindProperClosestLocation();
Console.WriteLine($"The proper closest location is {properClosestLocation}.");
