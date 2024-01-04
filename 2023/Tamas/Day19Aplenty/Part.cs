namespace Day19Aplenty;

internal readonly struct Part
{
    private readonly int[] attributes;

    public Part(
        int x,
        int m,
        int a,
        int s)
    {
        attributes = new int[] { x, m, a, s };
    }

    public readonly int this[Attribute attribute] => 
        attributes[(int)attribute];

    public readonly int AttributeSum => 
        attributes.Sum();

    public static Part Parse(
        string input)
    {
        var parts = input[1..^1].Split(',');
        return new Part(
            int.Parse(parts[0][2..]),
            int.Parse(parts[1][2..]),
            int.Parse(parts[2][2..]),
            int.Parse(parts[3][2..]));
    }
}
