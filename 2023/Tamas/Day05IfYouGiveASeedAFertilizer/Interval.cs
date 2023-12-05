namespace Day05IfYouGiveASeedAFertilizer;

internal record struct Interval(
    uint Start,
    uint Length)
{
    public static readonly Interval Empty = new(0, 0);

    public readonly bool IsEmpty => Length == 0;
    public readonly uint End => Start + Length - 1;

    public readonly Interval GetIntersection(
        Interval other)
    {
        uint maybeStart = Math.Max(Start, other.Start);
        uint maybeEnd = Math.Min(End, other.End);
        return maybeStart <= maybeEnd
            ? new Interval(maybeStart, maybeEnd - maybeStart + 1)
            : Empty;
    }
}
