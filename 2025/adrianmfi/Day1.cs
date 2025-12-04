namespace adrianmfi;

public static class Day1
{
    static readonly string test = """
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
        """;

    public static int Part1()
    {
        var count = 0;
        var current = 50;
        // foreach (var item in test.Split('\n'))
        foreach (var item in File.ReadLines("inputs/day1.txt"))
        {
            var isIncrement = item[0] == 'R';
            var step = int.Parse(item[1..]) % 100;
            if (isIncrement)
            {
                current += step;
                current %= 100;
            }
            else
            {
                if (step > current)
                {
                    current += 100;
                }
                current -= step;
            }
            if (current == 0)
            {
                count++;
            }
        }

        return count;
    }

    public static int Part2()
    {
        var count = 0;
        var current = 50;
        // foreach (var item in test.Split('\n'))
        foreach (var item in File.ReadLines("inputs/day1.txt"))
        {
            var previous = current;
            var isIncrement = item[0] == 'R';
            var step = int.Parse(item[1..]);
            count += step / 100;
            step %= 100;
            if (isIncrement)
            {
                current += step;
                current %= 100;
            }
            else
            {
                if (step > current)
                {
                    current += 100;
                }
                current -= step;
            }

            if (current == 0 || previous != 0 && ((isIncrement && current < previous) || (!isIncrement && current > previous)))
            {
                count++;
            }
        }

        return count;
    }


}
