using System.Text;
using System.Text.RegularExpressions;

namespace Aoc2024;

using Position = (int row, int col);

public static class Problem9
{
    private const int FreeSpace = -1;

    public static long Part1()
    {
        var data = File.ReadAllText("data/problem9.txt");
        var disk = new List<int>();
        var isSpace = false;
        var seqNo = 0;
        foreach (var marker in data.Trim())
        {
            var num = marker - '0';
            disk.AddRange(Enumerable.Repeat(isSpace ? FreeSpace : seqNo, num));

            if (!isSpace)
            {
                seqNo++;
            }

            isSpace = !isSpace;
        }

        var freeSpaceIndex = disk.IndexOf(FreeSpace);
        var compactorIndex = disk.FindLastIndex(entry => entry >= 0);
        while (freeSpaceIndex < compactorIndex)
        {
            disk[freeSpaceIndex] = disk[compactorIndex];
            disk[compactorIndex] = FreeSpace;
            freeSpaceIndex = disk.IndexOf(FreeSpace, freeSpaceIndex);
            do
            {
                compactorIndex--;
            } while (disk[compactorIndex] == FreeSpace);
        }

        return disk.Slice(0, freeSpaceIndex).Select((val, index) => (long)val * index).Sum();
    }


    class Space(int startIndex, int size)
    {
        public int StartIndex { get; set; } = startIndex;
        public int Size { get; set; } = size;
    }

    public static long Part2()
    {
        var data = File.ReadAllText("data/problem9.txt");
        var disk = new List<int>();
        var freeSpaces = new List<Space>();
        var isSpace = false;
        var seqNo = 0;
        foreach (var marker in data.Trim())
        {
            var num = marker - '0';
            if (isSpace && num > 0)
            {
                freeSpaces.Add(new Space(disk.Count, num));
            }

            disk.AddRange(Enumerable.Repeat(isSpace ? FreeSpace : seqNo, num));

            if (!isSpace)
            {
                seqNo++;
            }

            isSpace = !isSpace;
        }

        var compactorSeq = seqNo - 1;
        while (compactorSeq > 0)
        {
            var compactorStartIndex = disk.IndexOf(compactorSeq);
            var compactorEndIndex = disk.LastIndexOf(compactorSeq);
            var fileSize = compactorEndIndex - compactorStartIndex + 1;
            var fittingFreeSpaceIndex = freeSpaces.FindIndex(space => space.Size >= fileSize);
            if (fittingFreeSpaceIndex != -1)
            {
                var fittingFreeSpace = freeSpaces[fittingFreeSpaceIndex];
                if (fittingFreeSpace.StartIndex > compactorStartIndex)
                {
                    compactorSeq--;
                    // don't move to the right
                    continue;
                }
                // move file 
                for (int i = 0; i < fileSize; i++)
                {
                    disk[fittingFreeSpace.StartIndex + i] = disk[compactorStartIndex + i];
                    disk[compactorStartIndex + i] = FreeSpace;
                }

                if (fittingFreeSpace.Size == fileSize)
                {
                    freeSpaces.RemoveAt(fittingFreeSpaceIndex);
                }
                else
                {
                    fittingFreeSpace.Size -= fileSize;
                    fittingFreeSpace.StartIndex += fileSize;
                }
            }

            compactorSeq--;
        }

        return disk.Select((val, index) => val == FreeSpace ? 0 : (long)val * index).Sum();
    }
}