namespace Day13PointOfIncidence;

internal interface IPatternView
{
    int Height { get; }
    int Width { get; }
    char this[int row, int column] { get; }
}
