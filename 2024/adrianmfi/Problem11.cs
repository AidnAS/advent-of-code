using System.Text;
using System.Text.RegularExpressions;

namespace Aoc2024;

public static class Problem11
{
    public static int Part1()
    {
        var data = "3028 78 973951 5146801 5 0 23533 857";
        var stones = data.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToList();
        for (int i = 0; i < 25; i++)
        {
            for (int j = 0; j < stones.Count; j++)
            {
                var stone = stones[j];
                var stoneString = stone.ToString();
                if (stone == 0)
                {
                    stones[j] = 1;
                }
                else if (stoneString.Length % 2 == 0)
                {
                    var left = int.Parse(stoneString[(stoneString.Length / 2)..]);
                    var right = int.Parse(stoneString[..(stoneString.Length / 2)]);
                    stones[j] = left;
                    stones.Insert(j, right);
                    j++;
                }
                else
                {
                    stones[j] = stone * 2024;
                }
            }
        }

        return stones.Count;
    }

    public static long Part2()
    {
        var data = "3028 78 973951 5146801 5 0 23533 857";

        var stones = data.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).Select(num => (num, 1L)).ToDictionary();
        for (int i = 0; i < 75; i++)
        {
            foreach (var stoneKvp in stones.ToList())
            {
                var stone = stoneKvp.Key;
                var stoneCount = stoneKvp.Value;
                if (stoneCount == 0)
                {
                    continue;
                }
                var stoneString = stone.ToString();
                if (stone == 0)
                {
                    if (!stones.TryAdd(1, stoneCount))
                    {
                        stones[1] += stoneCount;
                    }
                }
                else if (stoneString.Length % 2 == 0)
                {
                    var left = int.Parse(stoneString[(stoneString.Length / 2)..]);
                    var right = int.Parse(stoneString[..(stoneString.Length / 2)]);
                    if (!stones.TryAdd(left, stoneCount))
                    {
                        stones[left] += stoneCount;
                    }
                    if (!stones.TryAdd(right, stoneCount))
                    {
                        stones[right] += stoneCount;
                    }
                }
                else
                {
                    if (!stones.TryAdd(stone * 2024, stoneCount))
                    {
                        stones[stone * 2024] += stoneCount;
                    } }

                stones[stone] -= stoneCount;
            }
        }


        return stones.Sum(s => s.Value);
    }
}