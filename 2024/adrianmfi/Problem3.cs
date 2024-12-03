using System.Text.RegularExpressions;

namespace Aoc2024;

public static class Problem3
{
    public static int Part1()
    {
        var data = File.ReadAllText("data/problem3.txt");
        var matches = Regex.Matches(data, "mul\\(?(\\d+),?(\\d+)\\)");
        return matches.Sum(match => int.Parse(match.Groups[1].Value) * int.Parse(match.Groups[2].Value));
    }

    public static int Part2()
    {
        var data = File.ReadAllText("data/problem3.txt");
        var splits = Regex.Matches(data, "do\\(\\)|don't\\(\\)");
        bool enabled = true;
        int startIdx = 0;
        int sum = 0;
        for (int i = 0; i < splits.Count; i++)
        {
            var splitMatch = splits[i];
            if (enabled)
            {
                var split = data.Substring(startIdx, splitMatch.Index - startIdx);
                var matches = Regex.Matches(split, "mul\\(?(\\d+),?(\\d+)\\)");
                sum += matches.Sum(match => int.Parse(match.Groups[1].Value) * int.Parse(match.Groups[2].Value));
            }

            startIdx = splitMatch.Index;
            enabled = splitMatch.Value == "do()";
        }

        if (enabled)
        {
            var finalSplit = data.Substring(startIdx);
            var finalMatches = Regex.Matches(finalSplit, "mul\\(?(\\d+),?(\\d+)\\)");
            sum += finalMatches.Sum(match => int.Parse(match.Groups[1].Value) * int.Parse(match.Groups[2].Value));
        }

        return sum;
    }
}