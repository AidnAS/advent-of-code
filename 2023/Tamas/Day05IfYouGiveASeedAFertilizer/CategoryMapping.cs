namespace Day05IfYouGiveASeedAFertilizer;

internal sealed record CategoryMapping(
    string Heading,
    IReadOnlyList<Rule> Rules)
{
    public uint GetDestination(uint source)
    {
        int ruleCount = Rules.Count;
        int min = 0;
        int max = ruleCount - 1;
        while (min <= max)
        {
            int current = (min + max) / 2;
            var rule = Rules[current];
            if (rule.Matches(source, out uint? destination))
            {
                return destination.Value;
            }

            if (source > rule.SourceStart)
            {
                min = current + 1;
            }
            else
            {
                max = current - 1;
            }
        }
        return source;
    }

    public IEnumerable<Interval> GetDestinationIntervals(
        IEnumerable<Interval> sources)
    {
        int ruleCount = Rules.Count;
        foreach (var source in sources)
        {
            int min = 0;
            int max = ruleCount - 1;
            int? firstMatching = null;
            while (min <= max)
            {
                int current = (min + max) / 2;
                var rule = Rules[current];
                if (rule.Matches(source, out _, out _))
                {
                    firstMatching = current;
                    if (source.Start >= rule.SourceStart)
                    {
                        break;
                    }
                }

                if (source.Start > rule.SourceStart)
                {
                    min = current + 1;
                }
                else
                {
                    max = current - 1;
                }
            }

            if (!firstMatching.HasValue)
            {
                yield return source;
                continue;
            }

            var remainingSource = source;
            for (int i = firstMatching.Value; i < Rules.Count; i++)
            {
                var rule = Rules[i];
                if (!rule.Matches(remainingSource, out var matchedSource, out var destination))
                {
                    yield return remainingSource;
                    break;
                }

                if (remainingSource.Start < matchedSource.Start)
                {
                    yield return new Interval(remainingSource.Start, matchedSource.Start - 1);
                }
                yield return destination;
                if (matchedSource.End == remainingSource.End)
                {
                    break;
                }
                remainingSource = new Interval(matchedSource.End + 1, remainingSource.End);
            }
        }
    }
}
