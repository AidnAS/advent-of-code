using System.Collections;
using System.Diagnostics.CodeAnalysis;

namespace Day14ParabolicReflectorDish;

internal sealed class BitArrayEqualityComparer : IEqualityComparer<BitArray>
{
    public static readonly BitArrayEqualityComparer Default = new();

    public bool Equals(
        BitArray? x, 
        BitArray? y)
    {
        if (x == null && y == null)
        {
            return true;
        }
        if (x == null || y == null)
        {
            return false;
        }

        if (x.Count != y.Count)
        {
            return false;
        }

        int numberArrayLength = (x.Count / 32) + 1;
        int[] xNumberArray = new int[numberArrayLength];
        x.CopyTo(xNumberArray, 0);
        int[] yNumberArray = new int[numberArrayLength];
        y.CopyTo(yNumberArray, 0);

        for (int i = 0; i < numberArrayLength; i++)
        {
            if (xNumberArray[i] != yNumberArray[i])
            {
                return false;
            }
        }
        return true;
    }

    public int GetHashCode(
        [DisallowNull] BitArray obj)
    {
        throw new NotImplementedException();
    }
}
