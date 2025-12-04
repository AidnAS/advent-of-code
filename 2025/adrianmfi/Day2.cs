

namespace adrianmfi;

public static class Day2
{
    static readonly string test = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    public static long Part1()
    {
        long result = 0;
        List<Range> ranges = [];

        // foreach (var item in test.Split(','))
        foreach (var item in File.ReadLines("inputs/day2.txt").First().Split(','))
        {
            var rangeStr = item.Split('-');
            ranges.Add(new(long.Parse(rangeStr[0]), long.Parse(rangeStr[1])));
        }
        var maxVal = ranges.MaxBy(range => range.Max)!.Max;
        long n = 0;
        while (true)
        {
            var numDigits = Math.Floor(Math.Log10(n) + 1);
            long test = n * (long)Math.Pow(10, numDigits) + n;
            if (test > maxVal)
            {
                break;
            }

            if (ranges.Any(range => range.Includes(test)))
            {
                result += test;
            }
            n++;
        }
        return result;

    }
    public static long Part2()
    {
        List<Range> ranges = [];

        // foreach (var item in test.Split(','))
        foreach (var item in File.ReadLines("inputs/day2.txt").First().Split(','))
        {
            var rangeStr = item.Split('-');
            ranges.Add(new(long.Parse(rangeStr[0]), long.Parse(rangeStr[1])));
        }
        var maxVal = ranges.MaxBy(range => range.Max)!.Max;

        var candidates = GetCandidatesPt2(maxVal);

        return candidates.Where(c => ranges.Any(r => r.Includes(c))).Sum();

    }

    private static List<long> GetCandidatesPt2(long maxVal)
    {
        HashSet<long> set = [];
        long pattern = 1;
        while (true)
        {
            var numDigits = Math.Floor(Math.Log10(pattern) + 1);
            long test = pattern * (long)Math.Pow(10, numDigits) + pattern;
            if (test > maxVal)
            {
                break;
            }

            while (true)
            {
                set.Add(test);
                test = test * (long)Math.Pow(10, numDigits) + pattern;
                if (test > maxVal)
                {
                    break;
                }
            }
            pattern++;
        }
        return [.. set];
    }
}


record Range(long Min, long Max)
{
    public bool Includes(long num)
    {
        return num >= Min && num <= Max;
    }
}
