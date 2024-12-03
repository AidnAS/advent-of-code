namespace Aoc2024;

public static class Problem2
{
    public static int Part1()
    {
        var rowCount = 0;
        foreach (var line in File.ReadLines("data/problem2.txt"))
        {
            var parts = line.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray();
            var diffs = new int[parts.Length - 1];
            for (var i = 0; i < parts.Length - 1; i++)
            {
                diffs[i] = parts[i + 1] - parts[i];
            }

            if (diffs.All(diff => diff is > 0 and <= 3) || diffs.All(diff => diff is < 0 and >= -3))
            {
                Console.WriteLine($"{line}");
                rowCount++;
            }
        }

        return rowCount;
    }

    public static int Part2()
    {
        var rowCount = 0;
        foreach (var line in File.ReadLines("data/problem2.txt"))
        {
            var parts = line.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray();
            if (IsValidSequence(parts))
            {
                rowCount++;
            }
            else
            {
                for (var skip = 0; skip < parts.Length; skip++)
                {
                    var newParts = parts.Where((_, i) => i != skip).ToArray();
                    if (IsValidSequence(newParts))
                    {
                        rowCount++;
                        break;
                    }
                }
            }
        }

        return rowCount;
    }


    private static bool IsValidSequence(int[] nums)
    {
        var diffs = Enumerable.Range(0, nums.Length - 1)
            .Select(i => nums[i + 1] - nums[i])
            .ToArray();

        return diffs.All(d => d is > 0 and <= 3) ||
               diffs.All(d => d is < 0 and >= -3);
    }
}