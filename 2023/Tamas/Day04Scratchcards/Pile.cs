namespace Day04Scratchcards;

internal sealed class Pile
{
    private readonly IReadOnlyList<ScratchCard> cards;

    public Pile(
        IReadOnlyList<ScratchCard> cards)
    {
        this.cards = cards;
    }

    public static Pile ReadFrom(string fileName)
    {
        var allLines = File.ReadAllLines(fileName);
        var cards = new List<ScratchCard>(allLines.Length);
        foreach (var line in allLines)
        {
            if (line.Length == 0)
            {
                continue;
            }

            cards.Add(
                ScratchCard.Parse(line));
        }
        
        return new Pile(cards);
    }

    public int GetSumPoints()
    {
        return cards
            .Sum(card => card.GetPointValue());
    }

    public long GetTotalCardCount()
    {
        var countArray = new long[cards.Count];
        Array.Fill(countArray, 1);
        for (int i = 0; i < cards.Count; i++)
        {
            var card = cards[i];
            long count = countArray[i];
            int matchCount = card.GetMatchingNumberCount();
            for (int j = 0; j < matchCount; j++)
            {
                countArray[i + 1 + j] += count;
            }
        }

        return countArray.Sum();
    }
}
