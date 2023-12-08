using Day07CamelCards;

string inputFile = "input.txt";
var simpleGame = Game.LoadFrom(inputFile, new SimpleCamelCardRules());
int totalWinningsSimple = simpleGame.CalculateTotalWinnings();
Console.WriteLine($"The total winnings with simple rules is {totalWinningsSimple}.");

var gameWithJoker = Game.LoadFrom(inputFile, new CamelCardRulesWithJoker());
int totalWinningsWithJoker = gameWithJoker.CalculateTotalWinnings();
Console.WriteLine($"The total winnings with rules with joker is {totalWinningsWithJoker}.");