namespace Day13PointOfIncidence;

internal sealed class Pattern
{
    private const char Ash = '.';
    private const char Rocks = '#';

    private readonly IReadOnlyList<string> rows;

    public Pattern(
        IReadOnlyList<string> rows)
    {
        this.rows = rows;
    }

    public int GetSummaryValue(
        bool fixingSmudge)
    {
        var summaryValue = GetSummaryValue(
            new NormalPatternView(rows), 100, fixingSmudge);
        if (!summaryValue.HasValue)
        {
            summaryValue = GetSummaryValue(
                new MirroredPatternView(rows), 1, fixingSmudge);
        }
        return summaryValue!.Value;
    }

    private static int? GetSummaryValue(
        IPatternView view,
        int multiplier,
        bool fixingSmudge)
    {
        var rowNumbers = RowsToNumbers(view);
        int? rowBeforeMirror = null;
        for (int candidateRow = 0; candidateRow < rowNumbers.Count - 1; candidateRow++)
        {
            if (IsRowBeforeMirror(candidateRow, view.Height, rowNumbers, fixingSmudge))
            {
                rowBeforeMirror = candidateRow;
                break;
            }
        }

        return (rowBeforeMirror + 1) * multiplier;
    }

    private static bool IsRowBeforeMirror(
        int candidateRow,
        int height,
        IReadOnlyList<ulong> rowNumbers,
        bool fixingSmudge)
    {
        bool isSmudgeFixed = false;
        for (int topRow = candidateRow; topRow >= 0; topRow--)
        {
            int bottomRow = candidateRow + (candidateRow - topRow) + 1;
            if (bottomRow >= height)
            {
                break;
            }
            
            ulong xor = rowNumbers[topRow] ^ rowNumbers[bottomRow];
            if (xor != 0)
            {
                if (!fixingSmudge
                    || isSmudgeFixed
                    || ulong.PopCount(xor) != 1)
                {
                    return false;
                }

                isSmudgeFixed = true;
            }
        }
        
        return !fixingSmudge || isSmudgeFixed;
    }

    private static IReadOnlyList<ulong> RowsToNumbers(
        IPatternView view)
    {
        if (view.Width > 64)
        {
            throw new InvalidOperationException();
        }

        var numbers = new ulong[view.Height];
        for (int row = 0; row < view.Height; row++)
        {
            ulong number = 0;
            for (int column = 0; column < view.Width; column++)
            {
                number <<= 1;

                char cell = view[row, column];
                if (cell == Rocks)
                {
                    number += 1;
                }
            }
            numbers[row] = number;
        }
        return numbers;
    }
}
