

namespace adrianmfi;

public static class Day3
{
    static readonly string test = """
    987654321111111
    811111111111119
    234234234234278
    818181911112111
    """;
    public static int Part1()
    {
        int result = 0;

        // foreach (var line in test.Split('\n'))
        foreach (var line in File.ReadLines("inputs/day3.txt"))
        {
            var maxDigit = line[..^1].Max();
            var secondMaxDigit = line[(line.IndexOf(maxDigit) + 1)..].Max();
            result += int.Parse($"{maxDigit}{secondMaxDigit}");
        }

        return result;

    }

    public static long Part2()
    {
        long result = 0;

        // foreach (var line in test.Split('\n'))
        foreach (var line in File.ReadLines("inputs/day3.txt"))
        {
            var joltage = "";
            var currentOffset = 0;
            for (int i = 0; i < 12; i++)
            {
                var maxDigit = line[currentOffset..^(11 - i)].Max();
                currentOffset = line.IndexOf(maxDigit, currentOffset) + 1;
                joltage += maxDigit;
            }
            result += long.Parse(joltage);
        }

        return result;

    }
}
