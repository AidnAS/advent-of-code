namespace Day07CamelCards;

internal sealed class Game
{
    private readonly IReadOnlyList<Player> players;

    private Game(
        IReadOnlyList<Player> players)
    {
        this.players = players;
    }

    public int CalculateTotalWinnings()
    {
        int totalWinnings = 0;
        for (int i = 0; i < players.Count; i++)
        {
            int rank = i + 1;
            var player = players[i];
            totalWinnings += rank * player.Bid;
        }
        return totalWinnings;
    }

    public static Game LoadFrom(
        string path,
        ICamelCardRules rules)
    {
        var allLines = File.ReadAllLines(path);
        var players = new List<Player>(allLines.Length);
        foreach (var line in allLines)
        {
            if (line.Length == 0)
            {
                continue;
            }

            players.Add(
                new Player(
                    rules.Evaluate(line[..5]),
                    int.Parse(line[6..])));
        }

        players.Sort(PlayerHandStrengthComparer.Default);
        return new Game(players);
    }
}
