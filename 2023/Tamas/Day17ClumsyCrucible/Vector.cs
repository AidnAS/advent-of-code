namespace Day17ClumsyCrucible;

internal readonly record struct Vector(
    int Row,
    int Column)
{
    public static readonly Vector Null = new();
    public static readonly Vector Up = new(-1, 0);
    public static readonly Vector Right = new(0, 1);
    public static readonly Vector Down = new(1, 0);
    public static readonly Vector Left = new(0, -1);

    public static readonly IReadOnlyList<Vector> Directions = new Vector[] { Up, Right, Down, Left };

    public readonly Vector Reverse => this * -1;

    public static Vector operator +(
        Vector left, 
        Vector right)
    {
        return new Vector(
            left.Row + right.Row, 
            left.Column + right.Column);
    }

    public static Vector operator *(
        Vector vector,
        int m)
    {
        return new Vector(
            vector.Row * m, 
            vector.Column * m);
    }

    public override string ToString()
    {
        return $"{Row},{Column}";
    }
}
