using System.Text.RegularExpressions;

namespace Day02CubeConundrum;

internal sealed partial class Records
{
    public IReadOnlyList<Game> Games { get; private init; } = default!;

    public static Records ParseFrom(string fileName)
    {
        var allLines = File.ReadAllLines(fileName);
        var gameList = new List<Game>(allLines.Length);
        var colorCountList = new List<(int Count, Color Color)>(3);
        foreach (var line in allLines)
        {
            var mainParts = line.Split(':');
            if (mainParts.Length != 2)
            {
                continue;
            }
            
            var gameId = int.Parse(mainParts[0]["Game ".Length..]);

            var drawParts = mainParts[1].Split(';');
            var drawList = new List<Cubes>(drawParts.Length);
            foreach (var draw in drawParts)
            {
                var colorRecordParts = draw.Split(',', StringSplitOptions.TrimEntries);
                colorCountList.Clear();
                foreach (var colorRecord in colorRecordParts)
                {
                    var finalParts = colorRecord.Split(' ', StringSplitOptions.RemoveEmptyEntries);
                    var count = int.Parse(finalParts[0]);
                    var color = Parse(finalParts[1]);
                    colorCountList.Add((count, color));
                }
                drawList.Add(Cubes.FromList(colorCountList));
            }
            gameList.Add(new Game(gameId, drawList));
        }

        return new Records()
        {
            Games = gameList
        };
    }

    public int GetPossibleGameIdSum(
        Cubes limits)
    {
        return Games.Sum(game => game.IsPossible(limits) ? game.Id : 0);
    }

    public long GetSumPowerOfMinRequiredCubes()
    {
        return Games.Sum(game => game.GetMinimumRequiredCubes().Power);
    }

    private static Color Parse(string input)
    {
        if (input == "red") { return Color.Red; }
        if (input == "green") { return Color.Green; }
        if (input == "blue") { return Color.Blue; }
        throw new ArgumentOutOfRangeException();
    }
}
