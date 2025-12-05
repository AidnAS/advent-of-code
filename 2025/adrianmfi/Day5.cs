

namespace adrianmfi;

public static class Day5
{
    static readonly string test = """
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
    """;
    public static long Part1()
    {
        bool parseRange = true;
        List<Range> ranges = [];
        List<long> ids = [];
        // foreach (var line in test.Split('\n'))
        foreach (var line in File.ReadLines("inputs/day5.txt"))
        {
            if (line.IsWhiteSpace())
            {
                parseRange = false;
                continue;
            }
            if (parseRange)
            {
                var rangeStr = line.Split('-');
                ranges.Add(new(long.Parse(rangeStr[0]), long.Parse(rangeStr[1])));
            }
            else
            {
                ids.Add(long.Parse(line));
            }
        }



        return ids.Count(id => ranges.Any(range => range.Includes(id)));

    }

    public static long Part2()
    {
        List<Range> ranges = [];
        foreach (var line in test.Split('\n'))
        // foreach (var line in File.ReadLines("inputs/day5.txt"))
        {
            if (line.IsWhiteSpace())
            {
                break;
            }
            var rangeStr = line.Split('-');
            ranges.Add(new(long.Parse(rangeStr[0]), long.Parse(rangeStr[1])));
        }

        ranges.Sort((r1, r2) => r1.Min.CompareTo(r2.Min));


        List<Range> mergedRanges = [];
        int start = 0;
        while (start < ranges.Count - 1)
        {
            var currRange = ranges[start];
            int end = start + 1;
            while (end < ranges.Count)
            {
                var nextRange = ranges[end];
                if (!currRange.Overlaps(nextRange))
                {
                    break;
                }
                currRange = currRange.Merge(nextRange);
                end++;
            }
            mergedRanges.Add(currRange);
            start = end;
        }
        if (start == ranges.Count - 1)
        {
            mergedRanges.Add(ranges[start]);
        }

        return mergedRanges.Sum(range => range.Max + 1 - range.Min);

    }

    record Range(long Min, long Max)
    {
        public bool Includes(long num)
        {
            return num >= Min && num <= Max;
        }

        public bool Overlaps(Range other)
        {
            return Max >= other.Min && Min <= other.Max;
        }

        // Assumes overlap
        public Range Merge(Range other)
        {
            var newMin = Math.Min(Min, other.Min);
            var newMax = Math.Max(Max, other.Max);
            return new(newMin, newMax);
        }
    }

}
