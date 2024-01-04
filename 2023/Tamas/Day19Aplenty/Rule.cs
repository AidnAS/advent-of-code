namespace Day19Aplenty;

internal sealed class Rule
{
    private readonly int result;
    private readonly Attribute? attribute;
    private readonly Relation relation;
    private readonly int threshold;
    private readonly int relationAdjustedThreshold;

    private Rule(
        int result,
        Attribute? attribute = null,
        Relation relation = default,
        int threshold = default)
    {
        this.result = result;
        this.attribute = attribute;
        this.relation = relation;
        this.threshold = threshold;
        relationAdjustedThreshold = (int)relation * threshold;
    }

    public int Result => result;

    public static Rule CreateUnconditional(
        int result)
    {
        return new Rule(
            result);
    }

    public static Rule CreateConditional(
        int result,
        Attribute attribute,
        Relation relation,
        int threshold)
    {
        return new Rule(
            result, attribute, relation, threshold);
    }

    public int? Evaluate(
        Part part)
    {
        return !attribute.HasValue 
            || (int)relation * part[attribute.Value] < relationAdjustedThreshold
            ? result
            : null;
    }

    public (AttributeRanges? Pass, AttributeRanges? Fail) Evaluate(
        AttributeRanges ranges)
    {
        if (!attribute.HasValue)
        {
            return (ranges, null);
        }

        var range = ranges[attribute.Value];
        if (relation == Relation.LessThan)
        {
            if (threshold <= range.Start) { return (null, ranges); }
            else if (threshold > range.End) { return (ranges, null); }
            else
            {
                var fail = ranges.With(attribute.Value, new Range(threshold, range.End));
                ranges[attribute.Value] = new Range(range.Start, threshold - 1);
                return (ranges, fail);
            }
        }
        else
        {
            if (threshold < range.Start) { return (ranges, null); }
            else if (threshold >= range.End) { return (null, ranges); }
            else
            {
                var fail = ranges.With(attribute.Value, new Range(range.Start, threshold));
                ranges[attribute.Value] = new Range(threshold + 1, range.End);
                return (ranges, fail);
            }
        }
    }
}
