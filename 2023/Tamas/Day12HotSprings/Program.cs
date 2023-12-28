using System.Diagnostics;
using Day12HotSprings;

var springRowList = File.ReadAllLines("input.txt")
    .Where(line => line.Length > 0)
    .Select(SpringRow.Parse)
    .ToList();
long sum = CountDifferentArrangements(springRowList);
Console.WriteLine($"The number of different arrangements folded is {sum}.");

var stopwatch = Stopwatch.StartNew();
sum = CountDifferentArrangements(springRowList.Select(row=>row.Unfold(5)));
stopwatch.Stop();
Console.WriteLine(stopwatch.Elapsed);
Console.WriteLine($"The number of different arrangements unfolded is {sum}.");

static long CountDifferentArrangements(
    IEnumerable<SpringRow> rows)
{
    long sum = 0;
    foreach (var row in rows)
    {
        long countBranching = -1;
        int matchingCountBranching = -1;
        long countSplitting = -1;
        int matchingCountSplitting = -1;
        countBranching = row.CountDifferentArrangements(CountingStrategy.BranchingAtUnknowns);
        matchingCountBranching = row.MatchingCount;
        //countSplitting = row.CountDifferentArrangements(CountingStrategy.Splitting);
        //matchingCountSplitting = row.MatchingCount;
        //Console.WriteLine($"{row} => {countBranching}/{countSplitting} ({matchingCountBranching}/{matchingCountSplitting}-{row.SplittingIterationCount}-{row.SplittingFromulaCount})");
        sum += countBranching != -1 ? countBranching : countSplitting;
    }
    return sum;
}