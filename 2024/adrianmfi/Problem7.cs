using System.Text;
using System.Text.RegularExpressions;

namespace Aoc2024;

public static class Problem7
{
    public static long Part1()
    {
        var data = File.ReadAllText("data/problem7.txt");

        var lines = data.Split("\n", StringSplitOptions.RemoveEmptyEntries);
        long sum = 0;
        foreach (var line in lines)
        {
            var split = line.Split(":");
            var target = long.Parse(split[0]);
            var nums = split[1].Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToArray();
            var results = new long[nums.Length];
            results[0] = nums[0];
            var ops = new Operation[nums.Length - 1];
            for (int j = 0; j < ops.Length; j++)
            {
                ops[j] = Operation.Multiply;
            }

            int i = 1;

            while (true)
            {
                var op = ops[i - 1];
                results[i] = op == Operation.Multiply ? results[i - 1] * nums[i] : results[i - 1] + nums[i];


                if (i == nums.Length - 1 && results[i] == target)
                {
                    // found a match
                    sum += target;
                    break;
                }

                if (results[i] > target || i == nums.Length - 1)
                {
                    i = FindPreviousLocation(ops, i);
                    if (i == -1)
                    {
                        // All operations are add, no match
                        break;
                    }

                    // backtrack
                    ops[i - 1] = Operation.Add;
                    for (var j = i; j < ops.Length; j++)
                    {
                        ops[j] = Operation.Multiply;
                    }
                }
                else
                {
                    i++;
                }
            }
        }

        return sum;
    }

    private static int FindPreviousLocation(Operation[] appliedOps, int startI)
    {
        for (int j = startI; j >= 1; j--)
        {
            if (appliedOps[j - 1] != Operation.Add)

            {
                return j;
            }
        }

        return -1;
    }

    public static long Part2()
    {
        var data = File.ReadAllText("data/problem7.txt");

        var lines = data.Split("\n", StringSplitOptions.RemoveEmptyEntries);
        long sum = 0;
        foreach (var line in lines)
        {
            var split = line.Split(":");
            var target = long.Parse(split[0]);
            var nums = split[1].Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToArray();
            var results = new long[nums.Length];
            results[0] = nums[0];
            var ops = new Operation[nums.Length - 1];
            for (int j = 0; j < ops.Length; j++)
            {
                ops[j] = Operation.Multiply;
            }

            int i = 1;

            while (true)
            {
                var op = ops[i - 1];
                var res = op switch
                {
                    Operation.Multiply => results[i - 1] * nums[i],
                    Operation.Add => results[i - 1] + nums[i],
                    Operation.Concatenate => long.Parse(results[i - 1].ToString() + nums[i].ToString()),
                };
                results[i] = res;

                if (i == nums.Length - 1 && results[i] == target)
                {
                    // found a match
                    sum += target;
                    break;
                }

                if (results[i] > target || i == nums.Length - 1)
                {
                    i = FindPreviousLocation(ops, i);
                    if (i == -1)
                    {
                        // All operations are add, no match
                        break;
                    }

                    var nextOp = ops[i - 1] == Operation.Multiply ? Operation.Concatenate : Operation.Add;
                    // backtrack
                    ops[i - 1] = nextOp;
                    for (var j = i; j < ops.Length; j++)
                    {
                        ops[j] = Operation.Multiply;
                    }
                }
                else
                {
                    i++;
                }
            }
        }

        return sum;
    }


    enum Operation
    {
        Multiply,
        Concatenate,
        Add
    };
}