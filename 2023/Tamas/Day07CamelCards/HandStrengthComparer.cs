namespace Day07CamelCards;

internal sealed class HandStrengthComparer : IComparer<Hand>
{
    public static readonly HandStrengthComparer Default = new();

    public int Compare(
        Hand? x, 
        Hand? y)
    {
        if (x == null || y == null)
        {
            throw new NotSupportedException();
        }

        if (x.HandType != y.HandType)
        {
            return y.HandType.CompareTo(x.HandType);
        }

        return x.SecondaryOrderValue.CompareTo(y.SecondaryOrderValue);
    }
}
