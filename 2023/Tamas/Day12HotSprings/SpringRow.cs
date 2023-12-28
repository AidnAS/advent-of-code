using System.Diagnostics;

namespace Day12HotSprings;

internal class SpringRow
{
    private const char SpringOperational = '.';
    private const char SpringDamaged = '#';
    private const char SpringUnknown = '?';

    private readonly string springs;
    private readonly int[] groups;
    private readonly Dictionary<long, long> branchingCache;

    private SpringRow(
        string springs,
        int[] groups)
    {
        this.springs = springs;
        this.groups = groups;
        
        branchingCache = new Dictionary<long, long>();
    }

    public int SplittingIterationCount { get; private set; }
    public int SplittingFromulaCount { get; private set; }
    public int MatchingCount { get; private set; }

    public long CountDifferentArrangements(
        CountingStrategy strategy)
    {
        var springIterator = SpanIterator.For(springs.AsSpan());
        var groupIterator = SpanIterator.For((ReadOnlySpan<int>)groups);
        int unknownCount = CountUnknowns(springIterator);
        int minLengthNeeded = CalculateMinLengthNeeded(groupIterator);

        SplittingIterationCount = 0;
        SplittingFromulaCount = 0;
        MatchingCount = 0;
        branchingCache.Clear();

        return strategy switch
        {
            CountingStrategy.BranchingAtUnknowns =>
                CountDifferentArrangementsBranchingAtUnknowns(
                    springIterator, groupIterator, minLengthNeeded, unknownCount),
            CountingStrategy.Splitting =>
                CountDifferentArrangementsSplitting(
                    springIterator, groupIterator, minLengthNeeded, unknownCount),
            _ => throw new ArgumentOutOfRangeException()
        };
    }

    public SpringRow Unfold(
        int count)
    {
        return new SpringRow(
            string.Join('?', Enumerable.Repeat(springs, count)),
            Enumerable.Repeat(groups, count).SelectMany(i => i).ToArray());
    }

    public static SpringRow Parse(
        string line)
    {
        var parts = line.Split(' ');
        return new SpringRow(
            springs: parts[0],
            groups: parts[1].Split(',').Select(int.Parse).ToArray());
    }

    public override string ToString()
    {
        return $"{springs} {string.Join(',', groups)}";
    }

    private long CountDifferentArrangementsSplitting(
        SpanIterator<char> springs,
        SpanIterator<int> groups,
        int minLengthNeeded,
        int unknownCount)
    {
        if (!MatchPrefix(ref springs, ref groups, ref minLengthNeeded, ref unknownCount))
        {
            return 0;
        }
        springs = springs.GetReversed();
        groups = groups.GetReversed();
        if (!MatchPrefix(ref springs, ref groups, ref minLengthNeeded, ref unknownCount))
        {
            return 0;
        }

        return CountDifferentArrangementsSplittingRec(
            springs, groups, minLengthNeeded, unknownCount);
    }

    private long CountDifferentArrangementsSplittingRec(
        SpanIterator<char> springs,
        SpanIterator<int> groups,
        int minLengthNeeded,
        int unknownCount)
    {
        if (springs.Length == 0)
        {
            return 1;
        }

        Debug.Assert(groups.Length > 0);

        if (unknownCount == springs.Length)
        {
            // This is the point of splitting, to be able to use a formula instead of counting.
            SplittingFromulaCount++;
            int toDistribute = springs.Length - minLengthNeeded;
            Debug.Assert(toDistribute > 0);
            int slotCount = groups.Length + 1;
            return Factorial(slotCount + toDistribute - 1)
                / (Factorial(toDistribute) * Factorial(slotCount - 1));
        }

        SplittingIterationCount++;

        long arrangementCount = 0;
        int middleIndex = groups.Length / 2;
        int middleLength = groups[middleIndex];
        var groupsBefore = groups[..middleIndex].GetReversed();
        var groupsAfter = groups[(middleIndex + 1)..^0];
        int minBefore = CalculateMinLengthNeeded(groupsBefore);
        int minAfter = CalculateMinLengthNeeded(groupsAfter);
        int separatorBefore = groupsBefore.Length > 0 ? 1 : 0;
        int separatorAfter = groupsAfter.Length > 0 ? 1 : 0;
        int maxMiddleEndIndex = springs.Length - (minAfter + separatorAfter) - 1;
        int runLength = 0;
        for (int springIndex = minBefore + separatorBefore; springIndex <= maxMiddleEndIndex; springIndex++)
        {
            runLength = springs[springIndex] != SpringOperational
                ? runLength + 1
                : 0;
            int beforeIndex = springIndex - middleLength;
            int afterIndex = springIndex + 1;
            if (runLength < middleLength
                || (beforeIndex >= 0 && springs[beforeIndex] == SpringDamaged)
                || (afterIndex < springs.Length && springs[afterIndex] == SpringDamaged))
            {
                continue;
            }

            var springsBefore = springs[0..(beforeIndex - separatorBefore + 1)].GetReversed();
            var springsAfter = springs[(afterIndex + separatorAfter)..^0];
            int unknownCountBefore = CountUnknowns(springsBefore);
            int unknownCountAfter = CountUnknowns(springsAfter);
            var groupsBeforeLocal = groupsBefore;
            int minBeforeLocal = minBefore;
            var groupsAfterLocal = groupsAfter;
            int minAfterLocal = minAfter;
            if (!MatchPrefix(ref springsBefore, ref groupsBeforeLocal, ref minBeforeLocal, ref unknownCountBefore)
                || !MatchPrefix(ref springsAfter, ref groupsAfterLocal, ref minAfterLocal, ref unknownCountAfter))
            {
                continue;
            }

            arrangementCount +=
                CountDifferentArrangementsSplittingRec(springsBefore, groupsBeforeLocal, minBeforeLocal, unknownCountBefore)
                * CountDifferentArrangementsSplittingRec(springsAfter, groupsAfterLocal, minAfterLocal, unknownCountAfter);
        }

        return arrangementCount;
    }

    private bool MatchPrefix(
        ref SpanIterator<char> springs,
        ref SpanIterator<int> groups,
        ref int minLengthNeeded,
        ref int unknownCount)
    {
        long result = Match(
            ref springs,
            ref groups,
            ref minLengthNeeded,
            ref unknownCount,
            stopAtFirstBranching: true,
            assumeDamagedFirst: false);
        return result == 1;
    }

    private long CountDifferentArrangementsBranchingAtUnknowns(
        SpanIterator<char> springs,
        SpanIterator<int> groups,
        int minLengthNeeded,
        int unknownCount,
        bool assumeDamagedFirst = false)
    {
        long cacheKey = (((long)springs.Length) << 32) + groups.Length;
        if (!branchingCache.TryGetValue(cacheKey, out long arrangementCount))
        {
            arrangementCount = Match(
                ref springs,
                ref groups,
                ref minLengthNeeded,
                ref unknownCount,
                stopAtFirstBranching: false,
                assumeDamagedFirst);
            branchingCache[cacheKey] = arrangementCount;
        }
        return arrangementCount;
    }

    private long Match(
        ref SpanIterator<char> springs,
        ref SpanIterator<int> groups,
        ref int minLengthNeeded,
        ref int unknownCount,
        bool stopAtFirstBranching,
        bool assumeDamagedFirst)
    {
        int leeway = springs.Length - minLengthNeeded;
        if (leeway < 0)
        {
            return 0;
        }

        if (springs.Length == 0)
        {
            return 1;
        }

        MatchingCount++;

        long arrangementCount = 0;
        int springIndex = 0;
        int groupIndex = 0;

        int? expectedLength = groups.Length > 0 ? groups[0] : null;
        bool advanceGroup = false;
        int? actualLength = null;

        if (assumeDamagedFirst)
        {
            Debug.Assert(expectedLength.HasValue);
            springIndex++;
            actualLength = 1;
        }

        while (true)
        {
            if (advanceGroup)
            {
                minLengthNeeded = AdjustMinLengthNeeded(minLengthNeeded, expectedLength!.Value);
                actualLength = null;
                groupIndex++;
                expectedLength = groupIndex != groups.Length ? groups[groupIndex] : null;
                advanceGroup = false;
            }
            
            char spring = springs[springIndex];
            if (spring == SpringUnknown)
            {
                if (!actualLength.HasValue)
                {
                    if (!expectedLength.HasValue)
                    {
                        // No more damaged groups expected, stepping over assumed operational.
                        Debug.Assert(leeway > 0);
                        leeway--;
                    }
                    else if (leeway == 0)
                    {
                        // Must be a new group because there's no more space to waste.
                        actualLength = 1;
                    }
                    else
                    {
                        if (!CheckIfGroupFits(
                            springs, springIndex, expectedLength.Value))
                        {
                            // The next group doesn't fit here, so must be operational.
                            leeway--;
                        }
                        else
                        {
                            if (stopAtFirstBranching)
                            {
                                // Prefix matched.
                                break;
                            }
                            else
                            {
                                // Trying with damaged group in branch. 
                                arrangementCount += CountDifferentArrangementsBranchingAtUnknowns(
                                    springs[springIndex..],
                                    groups[groupIndex..],
                                    minLengthNeeded,
                                    unknownCount - 1,
                                    assumeDamagedFirst: true);
                                
                                // Continuing here assuming operational.
                                leeway--;
                            }
                        }
                    }
                }
                else if (actualLength == expectedLength!.Value)
                {
                    // Group finished, assuming operational.
                    advanceGroup = true;
                }
                else
                {
                    // Assuming damaged.
                    actualLength++;
                }

                unknownCount--;
            }
            else if (spring == SpringOperational)
            {
                if (!actualLength.HasValue)
                {
                    if (--leeway < 0)
                    {
                        // Not enough space left.
                        return arrangementCount;
                    }
                    else
                    {
                        // Stepping over operational.
                    }
                }
                else if (actualLength == expectedLength!.Value)
                {
                    // Group finished.
                    advanceGroup = true;
                }
                else
                {
                    // Group too short - no match.
                    return arrangementCount;
                }
            }
            else // if (spring == SpringDamaged)
            {
                if (!actualLength.HasValue)
                {
                    if (!expectedLength.HasValue)
                    {
                        // Unexpected group - no match.
                        return arrangementCount;
                    }
                    else
                    {
                        // Found new group.
                        actualLength = 1;
                    }
                }
                else if (actualLength == expectedLength!.Value)
                {
                    // Group too long - no match.
                    return arrangementCount;
                }
                else
                {
                    // Group continues.
                    actualLength++;
                }
            }

            springIndex++;
            if (springIndex == springs.Length)
            {
                break;
            }
        }

        if (actualLength.HasValue)
        {
            if (actualLength == expectedLength!.Value)
            {
                minLengthNeeded = AdjustMinLengthNeeded(minLengthNeeded, expectedLength.Value);
                groupIndex++;
            }
            else
            {
                // Group too short - no match.
                return arrangementCount;
            }
        }

        arrangementCount++;
        springs = springs[springIndex..];
        groups = groups[groupIndex..];
        return arrangementCount;

        static int AdjustMinLengthNeeded(
            int minLengthNeeded,
            int matchedGroupLength)
        {
            return Math.Max(minLengthNeeded - matchedGroupLength - 1, 0);
        }

        static bool CheckIfGroupFits(
            SpanIterator<char> springs,
            int springIndex,
            int groupLength)
        {
            springIndex++;
            for (int i = 1; i < groupLength; i++)
            {
                if (springs[springIndex++] == SpringOperational)
                {
                    return false;
                }
            }
            return springIndex == springs.Length 
                || springs[springIndex] != SpringDamaged;
        }
    }

    private static int CalculateMinLengthNeeded(
        SpanIterator<int> groups)
    {
        if (groups.Length == 0)
        {
            return 0;
        }
        
        int result = groups.Length - 1;
        for (int i = 0; i < groups.Length; i++)
        {
            result += groups[i];
        }
        return result;
    }

    private static int CountUnknowns(
        SpanIterator<char> springs)
    {
        int count = 0;
        for (int i = 0; i < springs.Length; i++)
        {
            if (springs[i] == SpringUnknown)
            {
                count++;
            }
        }
        return count;
    }

    private static long Factorial(int n)
    {
        long result = 1;
        for (int i = 2; i <= n; i++)
        {
            result *= i;
        }
        return result;
    }
}
