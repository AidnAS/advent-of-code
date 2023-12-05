using System.Diagnostics.CodeAnalysis;

namespace Day05IfYouGiveASeedAFertilizer;

internal sealed record Rule(
    uint DestinationStart,
    uint SourceStart,
    uint Length)
{
    public bool Matches(
        uint source,
        [NotNullWhen(true)]
        out uint? destination)
    {
        if (source >= SourceStart
            && source < SourceStart + Length)
        {
            destination = DestinationStart + (source - SourceStart);
            return true;
        }
        else
        {
            destination = null;
            return false;
        }
    }

    public bool Matches(
        Interval source,
        out Interval matchedSource,
        out Interval destination)
    {
        var wholeSource = new Interval(SourceStart, Length);
        matchedSource = source.GetIntersection(wholeSource);
        if (matchedSource.IsEmpty)
        {
            destination = Interval.Empty;
            return false;
        }

        destination = new(
            DestinationStart + (matchedSource.Start - SourceStart), 
            matchedSource.Length);
        return true;
    }
}
