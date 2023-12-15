namespace Day07CamelCards;

internal sealed class CamelCardRulesWithJoker : ICamelCardRules
{
    public Hand Evaluate(string cards)
    {
        int equalityCount = 0;
        int secondaryOrderValue = 0;
        int jokerCount = 0;
        for (int i = 0; i < cards.Length; i++)
        {
            char leftCard = cards[i];
            secondaryOrderValue *= EvaluationUtils.SecondaryOrderValueBase;
            secondaryOrderValue += GetCardValue(leftCard);

            if (leftCard == 'J')
            {
                jokerCount++;
                continue;
            }

            for (int j = i + 1; j < cards.Length; j++)
            {
                char rightCard = cards[j];
                if (leftCard == rightCard)
                {
                    equalityCount++;
                }
            }
        }

        var handTypeAsInt = (int)EvaluationUtils.GetHandTypeByEqualityCount(equalityCount);
        handTypeAsInt >>= jokerCount;
        if (handTypeAsInt == 0)
        {
            handTypeAsInt = (int)HandType.FiveOfAKind;
        }

        return new Hand(
            cards,
            (HandType)handTypeAsInt,
            secondaryOrderValue);
    }

    private static int GetCardValue(
        char card)
    {
        return card switch
        {
            'J' => 0,
            >= '2' and <= '9' => card - '2' + 1,
            'T' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => throw new ArgumentOutOfRangeException(
                nameof(card))
        };
    }
}
