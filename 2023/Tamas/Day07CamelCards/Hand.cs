namespace Day07CamelCards;

internal sealed record Hand(
    string Cards,
    HandType HandType,
    int SecondaryOrderValue);