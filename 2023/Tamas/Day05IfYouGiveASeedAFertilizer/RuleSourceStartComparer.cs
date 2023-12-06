namespace Day05IfYouGiveASeedAFertilizer;

internal sealed class RuleSourceStartComparer : IComparer<Rule>
{
    public static readonly RuleSourceStartComparer Default = new();

    public int Compare(
        Rule? x, 
        Rule? y)
    {
        if (x == null || y == null)
        {
            throw new NotSupportedException();
        }
        return x.SourceStart.CompareTo(y.SourceStart);
    }
}
