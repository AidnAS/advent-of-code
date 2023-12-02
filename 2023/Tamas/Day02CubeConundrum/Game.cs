namespace Day02CubeConundrum;

internal sealed record Game(
    int Id,
    IReadOnlyList<Cubes> DrawList)
{
    public bool IsPossible(Cubes limits)
    {
        foreach (var draw in DrawList)
        {
            if (draw.Red > limits.Red
                || draw.Green > limits.Green
                || draw.Blue > limits.Blue)
            {
                return false;
            }
        }
        return true;
    }

    public Cubes GetMinimumRequiredCubes()
    {
        var min = new Cubes(0, 0, 0);
        foreach (var draw in DrawList)
        {
            min = Cubes.GetColorwiseMax(min, draw);
        }
        return min;
    }
}
