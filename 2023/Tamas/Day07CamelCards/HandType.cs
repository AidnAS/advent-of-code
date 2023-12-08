namespace Day07CamelCards;

internal enum HandType
{
    HighCard = 0b10100,
    OnePair = 0b1010,
    TwoPair = 0b1000,
    ThreeOfAKind = 0b101,
    FullHouse = 0b100,
    FourOfAKind = 0b10,
    FiveOfAKind = 0b1
}
