using System.Globalization;

namespace Day18LavaductLagoon;

internal sealed class DigPlan
{
    private readonly IReadOnlyList<DigInstruction> instructions;

    public DigPlan(
        IReadOnlyList<DigInstruction> instructions)
    {
        this.instructions = instructions;
    }

    public long CalculateVolume(
        bool usingColor)
    {
        long length = 0;
        long accumulator = 0;
        long row = 0;
        long column = 0;
        for (int i = 0; i < instructions.Count; i++)
        {
            var (previousRow, previousColumn) = (row, column);
            var (direction, distance, color) = instructions[i];
            if (usingColor)
            {
                distance = long.Parse(color[..^1], NumberStyles.HexNumber);
                direction = color[5] switch
                {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                    _ => throw new InvalidOperationException()
                };
            }

            (row, column) = direction switch
            {
                'U' => (row - distance, column),
                'R' => (row, column + distance),
                'D' => (row + distance, column),
                'L' => (row, column - distance),
                _ => throw new InvalidOperationException()
            };
            length += distance;
            accumulator += previousRow * column - previousColumn * row;
        }
        return Math.Abs(accumulator / 2) // Inside area plus about half of the area of the edge.
            + length / 2 // The other "about half" of the edge.
            + 1; // The missing area from the four orphan convex corners.
    }

    public static DigPlan Load(
        string path)
    {
        var allLines = File.ReadAllLines(path);
        var instructions = new List<DigInstruction>(allLines.Length);
        foreach (var line in allLines)
        {
            if (line.Length == 0)
            {
                continue;
            }

            instructions.Add(new DigInstruction(
                Direction: line[0],
                Distance: long.Parse(line[2..^9]),
                Color: line[^7..^1]));
        }
        return new(instructions);
    }
}
