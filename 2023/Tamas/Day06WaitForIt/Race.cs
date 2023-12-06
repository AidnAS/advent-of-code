namespace Day06WaitForIt;

internal sealed record Race(
    long TimeLimit,
    long RecordDistance)
{
    public static Race Parse(
        string timeLimitString,
        string recordDistanceString)
    {
        return new Race(
            long.Parse(timeLimitString),
            long.Parse(recordDistanceString));
    }
}
