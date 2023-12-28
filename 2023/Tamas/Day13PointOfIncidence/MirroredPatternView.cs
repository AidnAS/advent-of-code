namespace Day13PointOfIncidence;

internal class MirroredPatternView : IPatternView
{
    private readonly IReadOnlyList<string> rows;

    public MirroredPatternView(
        IReadOnlyList<string> rows)
    {
        this.rows = rows;
    }

    public int Height => rows[0].Length;
    public int Width => rows.Count;

    public char this[int row, int column]
    {
        get => rows[column][row];
    }
}
