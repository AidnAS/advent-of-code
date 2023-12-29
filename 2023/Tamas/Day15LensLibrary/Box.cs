namespace Day15LensLibrary;

internal sealed class Box
{
    public Box(
        int boxNumber)
    {
        BoxNumber = boxNumber;
    }

    public int BoxNumber { get; }
    public LinkedList<Lens> Lenses { get; } = new();
}
