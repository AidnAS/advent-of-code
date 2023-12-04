using Day04Scratchcards;

var pile = Pile.ReadFrom("input.txt");
int sumPoints = pile.GetSumPoints();
Console.WriteLine($"All cards are worth {sumPoints} together.");
long totalCards = pile.GetTotalCardCount();
Console.WriteLine($"Total number of cards after adding up won cards is {totalCards}.");