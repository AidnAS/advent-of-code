namespace Aoc2024;

public static class Problem1
{
    public static int Part1()
    {
        var lhs = new List<int>();
        var rhs = new List<int>();

        foreach (var line in File.ReadLines("data/problem1.txt"))
        {
            var parts = line.Split("   ");
            lhs.Add(int.Parse(parts[0]));
            rhs.Add(int.Parse(parts[1]));
        }

        lhs.Sort();
        rhs.Sort();

        return lhs.Zip(rhs, (l, r) => Math.Abs(l - r)).Sum();
    }

    public static int Part2()
    {
        var nums = new List<int>();
        var freqs = new Dictionary<int, int>();

        foreach (var line in File.ReadLines("data/problem1.txt"))
        {
            var parts = line.Split("   ");
            var left = int.Parse(parts[0]);
            var right = int.Parse(parts[1]);

            nums.Add(left);
            freqs[right] = freqs.GetValueOrDefault(right, 0) + 1;
        }

        return nums.Sum(num => freqs.GetValueOrDefault(num, 0) * num);
    }
}