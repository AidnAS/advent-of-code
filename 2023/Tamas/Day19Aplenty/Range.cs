namespace Day19Aplenty;

internal readonly record struct Range(
    int Start,
    int End)
{
    public uint Length 
        => (uint)(End - Start + 1);

    public override string ToString()
    {
        return $"{Start}-{End}";
    }
}
