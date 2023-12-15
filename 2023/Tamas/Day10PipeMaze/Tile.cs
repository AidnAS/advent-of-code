namespace Day10PipeMaze;

internal static class Tile
{
    public const byte UpMask = 0b1000;
    public const byte RightMask = 0b0100;
    public const byte DownMask = 0b0010;
    public const byte LeftMask = 0b0001;

    public static readonly IReadOnlyList<byte> Masks = new byte[] { UpMask, RightMask, DownMask, LeftMask };

    public const byte Dot = 0b0000;
    public const byte Pipe = 0b1010;
    public const byte Dash = 0b0101;
    public const byte L = 0b1100;
    public const byte J = 0b1001;
    public const byte Seven = 0b0011;
    public const byte F = 0b0110;
}
