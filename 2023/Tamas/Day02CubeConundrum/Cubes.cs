namespace Day02CubeConundrum;

internal record struct Cubes(
    int Red,
    int Green,
    int Blue)
{
    public readonly long Power => (long)Red * Green * Blue;

    public static Cubes FromList(
        IEnumerable<(int Count, Color Color)> list)
    {
        int red = 0;
        int green = 0;
        int blue = 0;
        foreach (var (count, color) in list)
        {
            if (color == Color.Red) { red = count; }
            if (color == Color.Green) { green = count; }
            if (color == Color.Blue) { blue = count; }
        }

        return new Cubes(red, green, blue);
    }

    public static Cubes GetColorwiseMax(
        Cubes left,
        Cubes right)
    {
        return new Cubes(
            Math.Max(left.Red, right.Red),
            Math.Max(left.Green, right.Green),
            Math.Max(left.Blue, right.Blue));
    }
}
