namespace Day16TheFloorWillBeLava;

internal sealed class Contraption
{
    private readonly Tile[] grid;
    private readonly int width;
    private readonly int height;
    
    private readonly DirectionRegister[] lightDirectionMap;
    private readonly bool[] energizationMap;
    
    private readonly Dictionary<Cursor, Path> paths;

    private Contraption(
        Tile[] grid,
        int width,
        int height)
    {
        this.grid = grid;
        this.width = width;
        this.height = height;
        
        lightDirectionMap = new DirectionRegister[grid.Length];
        energizationMap = new bool[grid.Length];
        
        paths = new Dictionary<Cursor, Path>();
    }

    public int GetMaxEnergizedTiles()
    {
        ResetMaps();

        var searchStartList = new List<Cursor>();
        int lastRow = height - 1;
        int lastColumn = width - 1;
        for (int row = 0; row < height; row++)
        {
            searchStartList.Add(new Cursor(new Vector(row, 0), Direction.Right));
            searchStartList.Add(new Cursor(new Vector(row, lastColumn), Direction.Left));
        }
        for (int column = 0; column < height; column++)
        {
            searchStartList.Add(new Cursor(new Vector(0, column), Direction.Down));
            searchStartList.Add(new Cursor(new Vector(lastRow, column), Direction.Up));
        }

        var startList = new List<Cursor>();
        foreach (var start in searchStartList)
        {
            var (coordinates, direction) = start;
            if (lightDirectionMap[ToIndex(coordinates)].IsSet(Mirror(direction)))
            {
                // Reversing a ligth ray will not produce more energized tiles than the original ray did.
                continue;
            }

            CountEnergizedTiles(start);
            startList.Add(start);
        }

        int maxEnergizedCount = 0;
        foreach (var start in startList)
        {
            ResetMaps();
            int energizedCount = CountEnergizedTiles(start);
            maxEnergizedCount = Math.Max(energizedCount, maxEnergizedCount);
        }
        return maxEnergizedCount;
    }

    public int CountEnergizedTilesRightFromTopLeftCorner()
    {
        return CountEnergizedTiles(
            new Cursor(Vector.Null, Direction.Right));
    }

    private void ResetMaps()
    {
        Array.Clear(lightDirectionMap);
        Array.Clear(energizationMap);
    }

    private int CountEnergizedTiles(
        Cursor start)
    {
        int energizedCount = 0;
        var stack = new Stack<Cursor>();
        Cursor? pathStart = null;
        List<int>? pathIndexList = null;
        for (Cursor? next = start; next != null; next ??= (stack.Count > 0 ? stack.Pop() : null))
        {
            var current = next.Value;
            next = null;
            var (coordinates, direction) = current;

            if (!IsInGrid(coordinates))
            {
                RegisterPath(ref pathStart, ref pathIndexList, current);
                continue;
            }

            int gridIndex = ToIndex(coordinates);
            var lightRegister = lightDirectionMap[gridIndex];
            var arrivingFromDirection = Mirror(direction);
            if (lightRegister.IsSet(arrivingFromDirection))
            {
                RegisterPath(ref pathStart, ref pathIndexList, current);
                continue;
            }

            if (!energizationMap[gridIndex])
            {
                energizedCount++;
                energizationMap[gridIndex] = true;
            }

            lightRegister = lightRegister.Set(arrivingFromDirection);
            var tile = grid[gridIndex];

            if (IsSplitter(tile))
            {
                RegisterPath(ref pathStart, ref pathIndexList, current);
            }
            else
            {
                if (pathIndexList == null)
                {
                    if (paths.TryGetValue(current, out var path))
                    {
                        foreach (var pathIndex in path.Indices)
                        {
                            if (!energizationMap[pathIndex])
                            {
                                energizedCount++;
                                energizationMap[pathIndex] = true;
                            }
                        }
                        next = path.Next;
                        continue;
                    }

                    pathIndexList = new List<int>();
                    pathStart = current;
                }
                pathIndexList.Add(gridIndex);
            }

            var rules = tile.GetRulesFrom(arrivingFromDirection);
            foreach (var nextDirection in rules)
            {
                lightRegister = lightRegister.Set(nextDirection);
                var directionVector = Vector.Directions[(int)nextDirection];
                var nextCoordinates = coordinates + directionVector;
                var target = new Cursor(nextCoordinates, nextDirection);
                if (!next.HasValue)
                {
                    next = target;
                }
                else
                {
                    stack.Push(target);
                }
            }
            lightDirectionMap[gridIndex] = lightRegister;
        }

        return energizedCount;
    }

    public static Contraption Load(
        string path)
    {
        var allLines = File.ReadAllLines(path);
        int tileCount = allLines.Sum(line => line.Length);
        var grid = new Tile[tileCount];
        int index = 0;
        foreach (var line in allLines)
        {
            foreach (var c in line)
            {
                grid[index++] = ToTile(c);
            }
        }

        int width = allLines[0].Length;
        int height = tileCount / width;
        return new Contraption(grid, width, height);

        static Tile ToTile(
            char c)
        {
            return c switch
            {
                '.' => Tile.EmptySpace,
                '/' => Tile.ForwardMirror,
                '\\' => Tile.BackwardMirror,
                '|' => Tile.VerticalSplitter,
                _ => Tile.HorizontalSplitter
            };
        }
    }

    private void RegisterPath(
        ref Cursor? start,
        ref List<int>? indices,
        Cursor afterEnd)
    {
        if (!start.HasValue)
        {
            return;
        }

        RegisterPath(start.Value, indices!, afterEnd);

        start = null;
        indices = null;
    }

    private void RegisterPath(
        Cursor start,
        List<int> indices,
        Cursor afterEnd)
    {
        AddPath(start, afterEnd);

        var startDirectionMirrored = Mirror(start.Direction);
        var reverseAfterEnd = new Cursor(
            start.Coordinates + Vector.Directions[(int)startDirectionMirrored],
            startDirectionMirrored);
        var afterEndDirectionMirrored = Mirror(afterEnd.Direction);
        var reverseStart = new Cursor(
            afterEnd.Coordinates + Vector.Directions[(int)afterEndDirectionMirrored],
            afterEndDirectionMirrored);

        AddPath(reverseStart, reverseAfterEnd);

        void AddPath(
            Cursor pathStart,
            Cursor pathAfterEnd)
        {
            paths.Add(
                pathStart,
                new Path(indices!, IsInGrid(pathAfterEnd.Coordinates) ? pathAfterEnd : null));
        }
    }

    private bool IsInGrid(
        Vector coordinates)
    {
        return coordinates.Row >= 0 && coordinates.Column >= 0
            && coordinates.Row < height && coordinates.Column < width;
    }

    private static bool IsSplitter(
        Tile tile)
    {
        return tile == Tile.VerticalSplitter || tile == Tile.HorizontalSplitter;
    }

    private static Direction Mirror(
        Direction direction)
    {
        int asNumber = (int)direction;
        asNumber = (asNumber + 2) % 4;
        return (Direction)asNumber;
    }

    private int ToIndex(
        Vector coordinates)
    {
        return coordinates.Row * width + coordinates.Column;
    }

    private readonly record struct Cursor(
        Vector Coordinates,
        Direction Direction);

    private sealed record Path(
        IReadOnlyList<int> Indices,
        Cursor? Next);
}
