namespace Day13PointOfIncidence;

internal sealed class NormalPatternView : IPatternView
{
    private readonly IReadOnlyList<string> rows;

    public NormalPatternView(
        IReadOnlyList<string> rows)
    {
        this.rows = rows;
    }

    public int Height => rows.Count;
    public int Width => rows[0].Length;
    
    public char this[int row, int column] 
    {
        get => rows[row][column];
    }
}
