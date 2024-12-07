using System.Text;
using System.Text.RegularExpressions;

namespace Aoc2024;

public static partial class Problem4
{
    public static int Part1()
    {
        var data = File.ReadAllText("data/problem4.txt");
        var horizontal = data.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
        var vertical = new List<string>();
        for (var col = 0; col < horizontal[0].Length; col++)
        {
            var sb = new StringBuilder();
            foreach (var row in horizontal)
            {
                sb.Append(row[col]);
            }

            vertical.Add(sb.ToString());
        }

        var diagonal = new List<string>();
        for (int diag = -horizontal.Length + 1; diag < horizontal.Length; diag++)
        {
            var topLeftToBottomRight = new StringBuilder();
            var topRightToBottomLeft = new StringBuilder();
            var diagonalLength = horizontal.Length - Math.Abs(diag);
            for (int i = 0; i < diagonalLength; i++)
            {
                var col = diag < 0 ? i : i + diag;
                var row = diag < 0 ? i - diag : i;
                topLeftToBottomRight.Append(horizontal[row][col]);
                topRightToBottomLeft.Append(horizontal[row][horizontal.Length - col - 1]);
            }

            diagonal.Add(topLeftToBottomRight.ToString());
            diagonal.Add(topRightToBottomLeft.ToString());
        }

        var lines = horizontal.Concat(vertical).Concat(diagonal).ToArray();
        lines = lines.Concat(lines.Select(Reverse).ToArray()).ToArray();

        var counts = lines.Sum(line => Xmas().Count(line));

        return counts;
    }

    public static int Part2()
    {
        var data = File.ReadAllText("data/problem4.txt");
        var length = data.IndexOf(Environment.NewLine, StringComparison.Ordinal);
        data = data.ReplaceLineEndings("@");
        var p1 = $@"M\SS.{{{length - 2}}}\SA\S.{{{length - 2}}}M\SS";
        var p2 = $@"M\SM.{{{length - 2}}}\SA\S.{{{length - 2}}}S\SS";
        var p3 = $@"S\SM.{{{length - 2}}}\SA\S.{{{length - 2}}}S\SM";
        var p4 = $@"S\SS.{{{length - 2}}}\SA\S.{{{length - 2}}}M\SM";
        var matches = Regex.Count(data, $"(?=({p1}|{p2}|{p3}|{p4})).");

        return matches;
    }

    private static string Reverse(this string str)
    {
        var charArray = str.ToCharArray();
        Array.Reverse(charArray);
        return new string(charArray);
    }

    [GeneratedRegex("XMAS")]
    private static partial Regex Xmas();
}