using Day09MirageMaintenance;

var allLines = File.ReadAllLines("input.txt");
var sequences = allLines.Select(RealSequence.Parse).ToList();

int sumNext = sequences
    .Sum(s => s.ExtrapolateNextNumber());
Console.WriteLine($"The sum of all extrapolated next values is {sumNext}.");

int sumPrevious = sequences
    .Sum(s => s.ExtrapolatePreviousNumber());
Console.WriteLine($"The sum of all extrapolated previous values is {sumPrevious}.");
