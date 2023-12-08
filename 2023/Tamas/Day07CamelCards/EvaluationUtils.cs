namespace Day07CamelCards;

internal static class EvaluationUtils
{
    public const int SecondaryOrderValueBase = 13;

    public static HandType GetHandTypeByEqualityCount(
        int equalityCount)
    {
        return equalityCount switch
        {
            0 => HandType.HighCard,
            1 => HandType.OnePair,
            2 => HandType.TwoPair,
            3 => HandType.ThreeOfAKind,
            4 => HandType.FullHouse,
            6 => HandType.FourOfAKind,
            10 => HandType.FiveOfAKind,
            _ => throw new ArgumentOutOfRangeException(
                nameof(equalityCount))
        };
    }
}
