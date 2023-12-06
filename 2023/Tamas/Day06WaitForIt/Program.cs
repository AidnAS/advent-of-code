using Day06WaitForIt;

var allLines = File.ReadAllLines("input.txt");
var timeParts = allLines[0].Split(' ', StringSplitOptions.RemoveEmptyEntries);
var distanceParts = allLines[1].Split(' ', StringSplitOptions.RemoveEmptyEntries);
int raceCount = timeParts.Length - 1;
var allRaces = new List<Race>(raceCount);
for (int i = 0; i < raceCount; i++)
{
    int arrayIndex = i + 1;
    allRaces.Add(
        Race.Parse(
            timeParts[arrayIndex],
            distanceParts[arrayIndex]));
}

long answer = 1;
foreach (var race in allRaces)
{
    answer *= Calculator.FindNumberOfWaysToWin(race);
}

Console.WriteLine(
    $"The number of ways to win in each race multiplied is {answer}.");

var actualRace = Race.Parse(
    string.Concat(timeParts.Skip(1)),
    string.Concat(distanceParts.Skip(1)));
long numberOfWaysToWin = Calculator.FindNumberOfWaysToWin(actualRace);
Console.WriteLine(
    $"The number of ways to win the one race is {numberOfWaysToWin}.");
