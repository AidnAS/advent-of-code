using Day02CubeConundrum;

var records = Records.ParseFrom("input.txt");
int possibleSum = records.GetPossibleGameIdSum(
    new Cubes(12, 13, 14));
Console.WriteLine($"The sum of the IDs of possible games is {possibleSum}.");
long powerSum = records.GetSumPowerOfMinRequiredCubes();
Console.WriteLine($"The sum of the powers of the minimum required sets of cubes in each game is {powerSum}.");