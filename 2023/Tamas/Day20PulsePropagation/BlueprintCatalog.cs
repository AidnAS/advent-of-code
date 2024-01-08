using System.Text;

namespace Day20PulsePropagation;

internal static class BlueprintCatalog
{
    public static readonly Blueprint Int64;
    public static readonly Blueprint HighReset;

    static BlueprintCatalog()
    {
        Int64 = GetInt64();
        HighReset = GetHighReset();
    }

    private static Blueprint GetInt64()
    {
        var builder = new StringBuilder();
        for (int i = 0; i < 64; i++)
        {
            builder.AppendLine($"&i64o{i:D2}");
        }
        for (int i = 0; i < 63; i++)
        {
            builder.AppendLine($"%i64b{i:D2} -> i64b{i + 1:D2}, i64o{i:D2}");
        }
        builder.AppendLine($"%i64b63 -> i64o63");
        return new(
            builder.ToString(),
            inputName: "i64b00",
            outputName: "i64b63");
    }

    private static Blueprint GetHighReset()
    {
        var builder = new StringBuilder();
        builder.AppendLine("rout");
        builder.AppendLine("rdelay2 -> rout");
        builder.AppendLine("rdelay1 -> rdelay2");
        builder.AppendLine("&rinv -> rout");
        builder.AppendLine("&rcon -> rinv");
        builder.AppendLine("rnull -> rcon");
        builder.AppendLine("rin -> rdelay1, rcon");
        return new(builder.ToString(), "rin", "rout");
    }
}
