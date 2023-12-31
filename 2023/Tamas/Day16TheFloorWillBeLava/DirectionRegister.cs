using System.Collections;

namespace Day16TheFloorWillBeLava;

internal readonly record struct DirectionRegister(
    byte Flags
) : IEnumerable<Direction>
{
    public const byte UpFlags = 0b_0000_0001;
    public const byte RightFlags = 0b_0000_0010;
    public const byte DownFlags = 0b_0000_0100;
    public const byte LeftFlags = 0b_0000_1000;

    public static readonly IReadOnlyList<byte> AllDirectionFlags = new byte[] { UpFlags, RightFlags, DownFlags, LeftFlags };

    public bool IsSet(
        Direction direction)
    {
        return (Flags & AllDirectionFlags[(int)direction]) != 0;
    }

    public DirectionRegister Set(
        Direction direction)
    {
        return Flags | AllDirectionFlags[(int)direction];
    }

    public IEnumerator<Direction> GetEnumerator()
    {
        if ((Flags & UpFlags) != 0) { yield return Direction.Up; }
        if ((Flags & RightFlags) != 0) { yield return Direction.Right; }
        if ((Flags & DownFlags) != 0) { yield return Direction.Down; }
        if ((Flags & LeftFlags) != 0) { yield return Direction.Left; }
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }

    public static implicit operator DirectionRegister(int bits) 
        => new((byte)bits);
}
