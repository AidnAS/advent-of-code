namespace Day08HauntedWasteland;

internal sealed class Node
{
    public Node(
        string label)
    {
        Label = label;
    }

    public string Label { get; }
    public Node Left { get; set; } = null!;
    public Node Right { get; set; } = null!;

    public bool IsDestination { get; set; }
    public bool IsGhostDestination { get; set; }

    public Node GetNext(char direction)
    {
        return direction == 'L' ? Left : Right;
    }
}
