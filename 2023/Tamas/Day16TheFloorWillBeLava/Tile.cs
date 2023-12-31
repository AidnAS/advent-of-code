namespace Day16TheFloorWillBeLava;

internal readonly record struct Tile(
    ushort CombinedReflectionRules)
{
    public static readonly Tile EmptySpace = 0b_0010_0001_1000_0100;
    public static readonly Tile ForwardMirror = 0b_0001_0010_0100_1000;
    public static readonly Tile BackwardMirror = 0b_0100_1000_0001_0010;
    public static readonly Tile VerticalSplitter = 0b_0101_0001_0101_0100;
    public static readonly Tile HorizontalSplitter = 0b_0010_1010_1000_1010;

    public DirectionRegister GetRulesFrom(Direction direction)
        => CombinedReflectionRules >> ((int)direction * 4);

    public static implicit operator Tile(ushort combinedReflectionRules) 
        => new(combinedReflectionRules);
}
