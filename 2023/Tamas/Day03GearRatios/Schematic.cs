namespace Day03GearRatios;

internal sealed class Schematic
{
    private readonly string[] lines;
    private readonly int height;
    private readonly int width;

    private List<SchematicNumber>? parts;
    private List<Gear>? gears;

    private Schematic(
        string[] lines)
    {
        this.lines = lines;
        height = lines.Length;
        width = lines.Length > 0 ? lines[0].Length : 0;
    }

    public static Schematic ReadFrom(
        string fileName)
    {
        return new Schematic(
            File.ReadAllLines(fileName));
    }

    public char this[Location location]
    {
        get
        {
            var (row, column) = location;
            if (row < 0 || row >= height
                || column < 0 || column >= width)
            {
                return '.';
            }
            return lines[row][column];
        }
    }

    public IEnumerable<SchematicNumber> FindParts()
    {
        if (parts == null)
        {
            parts = GetNumbers()
               .Where(IsPart)
               .ToList();
        }
        return parts;
    }

    public IEnumerable<Gear> FindGears()
    {
        if (gears == null)
        {
            gears = new List<Gear>();
            var gearMap = new Dictionary<Location, HashSet<SchematicNumber>>();
            foreach (var part in FindParts())
            {
                MarkConnectedGears(part, gearMap);
            }

            foreach (var pair in gearMap)
            {
                var (location, connectedParts) = pair;
                if (connectedParts.Count != 2)
                {
                    continue;
                }

                gears.Add(
                    new Gear(
                        location,
                        connectedParts.Aggregate(
                            1, 
                            (acc, part) => acc * part.Value)));
            }
        }
        return gears;
    }

    private void MarkConnectedGears(
        SchematicNumber part,
        Dictionary<Location, HashSet<SchematicNumber>> gearMap)
    {
        foreach (var neighbour in GetNeighbours(part))
        {
            char c = this[neighbour];
            if (c != '*')
            {
                continue;
            }

            if (!gearMap.TryGetValue(neighbour, out var connectedParts))
            {
                connectedParts = new HashSet<SchematicNumber>();
                gearMap.Add(neighbour, connectedParts);
            }
            connectedParts.Add(part);
        }
    }

    private IEnumerable<SchematicNumber> GetNumbers()
    {
        // Instance of type 'ReadOnlySpan<char>' cannot be used inside a nested function,
        // query expression, iterator block or async method :(
        var numberList = new List<SchematicNumber>();

        for (int row = 0; row < height; row++)
        {
            var line = (ReadOnlySpan<char>)lines[row];
            int? start = null;
            for (int column = 0; column <= width; column++)
            {
                char c = column < width ? line[column] : '.';
                if (start.HasValue)
                {
                    if (!IsDigit(c))
                    {
                        int end = column - 1;
                        numberList.Add(
                            new SchematicNumber(
                                int.Parse(line.Slice(start.Value, end - start.Value + 1)),
                                row, start.Value, end));
                        start = null;
                    }
                }
                else
                {
                    if (IsDigit(c))
                    {
                        start = column;
                    }
                }
            }
        }
        return numberList;
    }

    private bool IsPart(
        SchematicNumber number)
    {
        return GetNeighbours(number)
            .Any(IsSymbol);
    }

    private static IEnumerable<Location> GetNeighbours(
        SchematicNumber number)
    {
        var (_, row, start, end) = number;
        yield return new Location(row, start - 1);
        yield return new Location(row, end + 1);
        for (int column = start - 1; column <= end + 1; column++)
        {
            yield return new Location(row - 1, column);
            yield return new Location(row + 1, column);
        }
    }

    private static bool IsDigit(
        char c)
    {
        return c >= '0' && c <= '9';
    }

    private bool IsSymbol(
        Location location)
    {
        return IsSymbol(this[location]);
    }


    private static bool IsSymbol(
        char c)
    {
        return !IsDigit(c) && c != '.';
    }
}
