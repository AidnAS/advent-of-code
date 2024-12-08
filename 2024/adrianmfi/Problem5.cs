using System.Text;
using System.Text.RegularExpressions;

namespace Aoc2024;

public static class Problem5
{
    public static int Part1()
    {
        var data = File.ReadAllText("data/problem5.txt");

        var parts = data.Split("\n\n");
        var rules = parts[0].Split("\n").Select(
                ruleStr =>
                {
                    var split = ruleStr.Split("|");
                    var lhs = int.Parse(split[0]);
                    var rhs = int.Parse(split[1]);
                    return (lhs, rhs);
                }
            ).GroupBy(rule => rule.lhs)
            .ToDictionary(group => group.Key, group => group.Select(rule => rule.rhs).ToArray());

        var updates = parts[1].Split("\n", StringSplitOptions.RemoveEmptyEntries).Select(
            updateStr =>
            {
                var split = updateStr.Split(",");
                return split.Select(int.Parse).ToArray();
            }
        ).ToArray();

        return updates.Sum(update =>
        {
            var invalid = new HashSet<int>();
            for (int i = update.Length - 1; i >= 0; i--)
            {
                var curr = update[i];
                if (invalid.Contains(curr))
                {
                    return 0;
                }

                if (rules.TryGetValue(curr, out var rule))
                {
                    invalid.UnionWith(rule);
                }
            }

            return update[update.Length / 2];
        });
    }

    public static int Part2()
    {
        var data = File.ReadAllText("data/problem5.txt");

        var parts = data.Split("\n\n");
        var rules = parts[0].Split("\n").Select(
                ruleStr =>
                {
                    var split = ruleStr.Split("|");
                    var lhs = int.Parse(split[0]);
                    var rhs = int.Parse(split[1]);
                    return (lhs, rhs);
                }
            ).GroupBy(rule => rule.lhs)
            .ToDictionary(group => group.Key, group => group.Select(rule => rule.rhs).ToArray());

        var updates = parts[1].Split("\n", StringSplitOptions.RemoveEmptyEntries).Select(
            updateStr =>
            {
                var split = updateStr.Split(",");
                return split.Select(int.Parse).ToArray();
            }
        ).ToArray();

        return updates.Where(update =>
        {
            var invalid = new HashSet<int>();
            for (int i = update.Length - 1; i >= 0; i--)
            {
                var curr = update[i];
                if (invalid.Contains(curr))
                {
                    return true;
                }

                if (rules.TryGetValue(curr, out var rule))
                {
                    invalid.UnionWith(rule);
                }
            }

            return false;
        }).Select(update => update.OrderBy(x => x, Comparer<int>.Create(
            (a, b) =>
            {
                if (rules.ContainsKey(a) && rules[a].Contains(b))
                {
                    return -1;
                }
                else if (rules.ContainsKey(b) && rules[b].Contains(a))
                {
                    return 1;
                }
                else
                {
                    return 0;
                }
            }
        )).ToArray()).Sum(update => update[update.Length / 2]);
    }
}