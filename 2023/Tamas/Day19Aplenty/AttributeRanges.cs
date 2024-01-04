namespace Day19Aplenty;

internal readonly struct AttributeRanges
{
    private readonly Range[] ranges;

    public AttributeRanges(
        Range x,
        Range m,
        Range a,
        Range s)
        : this(new Range[] { x, m, a, s })
    {
    }

    private AttributeRanges(
        Range[] ranges)
    {
        this.ranges = ranges;
    }

    public Range this[Attribute attribute]
    {
        get => ranges[(int)attribute];
        set
        {
            ranges[(int)attribute] = value;
        }
    }

    public ulong CombinationCount => 
        (ulong)ranges[0].Length 
        * ranges[1].Length 
        * ranges[2].Length 
        * ranges[3].Length;

    public AttributeRanges With(
        Attribute attribute,
        Range range)
    {
        var clone = new AttributeRanges(
            (Range[])ranges.Clone());
        clone[attribute] = range;
        return clone;
    }

    public override string ToString()
    {
        return $"x:{ranges[0]} m:{ranges[1]} a:{ranges[2]} s:{ranges[3]}";
    }
}
