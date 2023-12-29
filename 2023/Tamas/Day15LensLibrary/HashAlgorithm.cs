namespace Day15LensLibrary;

internal static class HashAlgorithm
{
    public static int Run(
        string input)
    {
        int value = 0;
        foreach (char c in input)
        {
            value = ((value + c) * 17) % 256;
        }
        return value;
    }
}
