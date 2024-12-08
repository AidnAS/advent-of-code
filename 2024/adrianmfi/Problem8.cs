using System.Text;
using System.Text.RegularExpressions;

namespace Aoc2024;

using Position = (int row, int col);

public static partial class Problem8
{
    public static long Part1()
    {
        var data = File.ReadAllText("data/problem8.txt");

        var antennaPositions = new Dictionary<char, List<Position>>();
        var lines = data.Split("\n", StringSplitOptions.RemoveEmptyEntries);
        for (int i = 0; i < lines.Length; i++)
        {
            var line = lines[i];

            var antennaMatches = Antennas().Matches(line);
            foreach (Match match in antennaMatches)
            {
                var position = new Position(i, match.Index);
                var frequency = match.Value[0];
                if (!antennaPositions.TryAdd(frequency, [position]))
                {
                    antennaPositions[frequency].Add(position);
                }
            }
        }

        var numRows = lines.Length;
        var numCols = lines[0].Length;

        var antinodePositions = new HashSet<Position>();

        foreach (var (_, positions) in antennaPositions)
        {
            for (int i = 0; i < positions.Count - 1; i++)
            {
                for (int j = i + 1; j < positions.Count; j++)
                {
                    var p1 = positions[i];
                    var p2 = positions[j];
                    var rowDiff = p2.row - p1.row;
                    var colDiff = p2.col - p1.col;

                    Position antinode1 = (p1.row - rowDiff, p1.col - colDiff);
                    Position antinode2 = (p2.row + rowDiff, p2.col + colDiff);

                    if (IsWithinBounds(antinode1))
                    {
                        antinodePositions.Add(antinode1);
                    }

                    if (IsWithinBounds(antinode2))
                    {
                        antinodePositions.Add(antinode2);
                    }
                }
            }
        }

        return antinodePositions.Count;

        bool IsWithinBounds(Position pos) =>
            pos switch
            {
                (< 0, _) => false,
                (_, < 0) => false,
                var (row, col) when row >= numRows || col >= numCols => false,
                _ => true
            };
    }


    public static long Part2()
    {
        var data = File.ReadAllText("data/problem8.txt");

        var antennaPositions = new Dictionary<char, List<Position>>();
        var lines = data.Split("\n", StringSplitOptions.RemoveEmptyEntries);
        for (int i = 0; i < lines.Length; i++)
        {
            var line = lines[i];

            var antennaMatches = Antennas().Matches(line);
            foreach (Match match in antennaMatches)
            {
                var position = new Position(i, match.Index);
                var frequency = match.Value[0];
                if (!antennaPositions.TryAdd(frequency, [position]))
                {
                    antennaPositions[frequency].Add(position);
                }
            }
        }

        var numRows = lines.Length;
        var numCols = lines[0].Length;

        var antinodePositions = new HashSet<Position>();

        foreach (var (_, positions) in antennaPositions)
        {
            for (int i = 0; i < positions.Count - 1; i++)
            {
                for (int j = i + 1; j < positions.Count; j++)
                {
                    var p1 = positions[i];
                    var p2 = positions[j];
                    var rowDiff = p2.row - p1.row;
                    var colDiff = p2.col - p1.col;

                    var currentPos = p1;
                    while (IsWithinBounds(currentPos))
                    {
                        antinodePositions.Add(currentPos);
                        currentPos = (currentPos.row - rowDiff, currentPos.col - colDiff);
                    }

                    currentPos = p2;
                    while (IsWithinBounds(currentPos))
                    {
                        antinodePositions.Add(currentPos);
                        currentPos = (currentPos.row + rowDiff, currentPos.col + colDiff);
                    }
                }
            }
        }

        return antinodePositions.Count;

        bool IsWithinBounds(Position pos) =>
            pos switch
            {
                (< 0, _) => false,
                (_, < 0) => false,
                var (row, col) when row >= numRows || col >= numCols => false,
                _ => true
            };
    }

    [GeneratedRegex("[a-zA-Z0-9]")]
    private static partial Regex Antennas();
}