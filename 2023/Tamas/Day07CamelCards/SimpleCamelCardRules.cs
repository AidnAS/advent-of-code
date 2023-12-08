namespace Day07CamelCards;

internal sealed class SimpleCamelCardRules : ICamelCardRules
{
    public Hand Evaluate(string cards)
    {
        int equalityCount = 0;
        int secondaryOrderValue = 0;
        for (int i = 0; i < cards.Length; i++)
        {
            char leftCard = cards[i];
            secondaryOrderValue *= EvaluationUtils.SecondaryOrderValueBase;
            secondaryOrderValue += GetCardValue(leftCard);

            for (int j = i + 1; j < cards.Length; j++)
            {
                char rightCard = cards[j];
                if (leftCard == rightCard)
                {
                    equalityCount++;
                }
            }
        }

        var handType = EvaluationUtils.GetHandTypeByEqualityCount(equalityCount);

        return new Hand(
            cards,
            handType,
            secondaryOrderValue);
    }

    private static int GetCardValue(
        char card)
    {
        return card switch
        {

            >= '2' and <= '9' => card - '2',
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => throw new ArgumentOutOfRangeException(nameof(card))
        };
    }
}
