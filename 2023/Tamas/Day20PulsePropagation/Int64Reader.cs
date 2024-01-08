namespace Day20PulsePropagation;

internal sealed class Int64Reader
{
    private const ulong HighBitOnly = 1ul << 63;
    private readonly FlipFlop[] bits;

    private Int64Reader(
        FlipFlop[] bits)
    {
        this.bits = bits;
    }

    public static Int64Reader Create(
        Circuit circuit,
        string prefix)
    {
        var bits = new FlipFlop[64];
        for (int i = 0; i < bits.Length; i++)
        {
            bits[i] = (FlipFlop)circuit[$"{prefix}i64b{i:D2}"];
        }
        return new Int64Reader(bits);
    }

    public ulong Read()
    {
        ulong result = 0;
        foreach (var bit in bits)
        {
            result >>= 1;
            if (bit.State)
            {
                result |= HighBitOnly;
            }
        }
        return result;
    }
}
