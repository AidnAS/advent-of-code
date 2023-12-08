namespace Day07CamelCards;

internal sealed class PlayerHandStrengthComparer : IComparer<Player>
{
    public static readonly PlayerHandStrengthComparer Default = new();

    public int Compare(
        Player? x, 
        Player? y)
    {
        return HandStrengthComparer.Default
            .Compare(x?.Hand, y?.Hand);
    }
}
