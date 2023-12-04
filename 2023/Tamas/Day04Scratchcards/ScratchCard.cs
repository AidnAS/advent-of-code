namespace Day04Scratchcards;

internal sealed class ScratchCard
{
    private static readonly int PrefixLength = "Card ".Length;
    private static readonly char[] MainSeparators = new char[] { ':', '|' };

    private readonly int cardId;
    private readonly IReadOnlyList<int> winningNumbers;
    private readonly IReadOnlyList<int> yourNumbers;

    private int? matchCount;

    public ScratchCard(
        int cardId,
        IReadOnlyList<int> winningNumbers,
        IReadOnlyList<int> yourNumbers)
    {
        this.cardId = cardId;
        this.winningNumbers = winningNumbers;
        this.yourNumbers = yourNumbers;
    }

    public static ScratchCard Parse(
        string line)
    {
        var parts = line.Split(MainSeparators);
        return new(
            cardId: int.Parse(parts[0][PrefixLength..]),
            winningNumbers: ParseNumberList(parts[1]),
            yourNumbers: ParseNumberList(parts[2]));

    }

    public int GetMatchingNumberCount()
    {
        if (!matchCount.HasValue)
        {
            matchCount = CalculateMatchingNumberCount();
        }
        return matchCount.Value;
    }

    public int GetPointValue()
    {
        int matchCount = GetMatchingNumberCount();
        return matchCount == 0
            ? 0
            : (int)Math.Pow(2, matchCount - 1);
    }

    private int CalculateMatchingNumberCount()
    {
        if (winningNumbers.Count == 0 || yourNumbers.Count == 0)
        {
            return 0;
        }

        int iw = 0;
        int iy = 0;
        int matchCount = 0;
        while (iw < winningNumbers.Count && iy < yourNumbers.Count)
        {
            int w = winningNumbers[iw];
            int y = yourNumbers[iy];
            if (w == y)
            {
                matchCount++;
                iw++;
                iy++;
            }
            else if (w < y)
            {
                iw++;
            }
            else
            {
                iy++;
            }
        }

        return matchCount;
    }

    private static IReadOnlyList<int> ParseNumberList(
        string input)
    {
        return input
            .Split(' ', StringSplitOptions.RemoveEmptyEntries)
            .Select(int.Parse)
            .OrderBy(n => n)
            .ToList();
    }
}
