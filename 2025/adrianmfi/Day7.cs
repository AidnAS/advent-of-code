

using System.Collections;
using System.Text.RegularExpressions;

namespace adrianmfi;

public static partial class Day7
{
    static readonly string test = """
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
    """;


    public static long Part1()
    {
        Position? startPosition = null;
        List<List<Position>> splitterRows = [];
        int rowNumber = 0;
        // foreach (var line in test.Split('\n'))
        foreach (var line in File.ReadLines("inputs/day7.txt"))
        {
            var startIndex = line.IndexOf('S');
            if (startIndex >= 0)
            {
                startPosition = new(rowNumber, startIndex);
            }
            var row = line.AllIndexesOf('^').Select(col => new Position(rowNumber, col)).ToList();
            if (row.Count > 0)
            {
                splitterRows.Add(row);
            }
            rowNumber++;
        }
        HashSet<int> currentBeams = [];
        currentBeams.Add(startPosition!.Col);
        int splitCount = 0;
        foreach (var row in splitterRows)
        {
            foreach (var splitter in row)
            {
                if (currentBeams.Contains(splitter.Col))
                {
                    splitCount++;
                    currentBeams.Remove(splitter.Col);
                    currentBeams.Add(splitter.Col - 1);
                    currentBeams.Add(splitter.Col + 1);
                }
            }
        }
        return splitCount;
    }

    public static long Part2()
    {
        Position? startPosition = null;
        List<List<Position>> splitterRows = [];
        int rowNumber = 0;
        // foreach (var line in test.Split('\n'))
        foreach (var line in File.ReadLines("inputs/day7.txt"))
        {
            var startIndex = line.IndexOf('S');
            if (startIndex >= 0)
            {
                startPosition = new(rowNumber, startIndex);
            }
            var row = line.AllIndexesOf('^').Select(col => new Position(rowNumber, col)).ToList();
            if (row.Count > 0)
            {
                splitterRows.Add(row);
            }
            rowNumber++;
        }
        Dictionary<int, long> currentBeams = [];
        currentBeams.Add(startPosition!.Col, 1);
        foreach (var row in splitterRows)
        {
            foreach (var splitter in row)
            {
                if (currentBeams.TryGetValue(splitter.Col, out long dedupCount))
                {
                    currentBeams.Remove(splitter.Col);
                    currentBeams[splitter.Col - 1] = currentBeams.TryGetValue(splitter.Col - 1, out var prevCount)
                        ? prevCount + dedupCount
                        : dedupCount;
                    currentBeams[splitter.Col + 1] = currentBeams.TryGetValue(splitter.Col + 1, out var nextCount)
                    ? nextCount + dedupCount
                    : dedupCount;
                }
            }
        }
        return currentBeams.Values.Sum();
    }

    record Position(int Row, int Col);

    private static int[] AllIndexesOf(this string s, char value)
    {
        List<int> res = [];
        int currentIndex = s.IndexOf(value);
        while (currentIndex != -1)
        {
            res.Add(currentIndex);
            currentIndex = s.IndexOf(value, currentIndex + 1);
        }
        return [.. res];

    }
}
