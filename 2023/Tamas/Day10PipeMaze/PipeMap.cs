namespace Day10PipeMaze;

internal sealed class PipeMap
{
    private readonly int width;
    private readonly int height;
    private readonly byte[] tiles;
    private readonly Vector start;

    private HashSet<Vector>? loop;

    private PipeMap(
        int width,
        int height,
        byte[] tiles,
        Vector start)
    {
        this.width = width;
        this.height = height;
        this.tiles = tiles;
        this.start = start;

        FixStart();
    }

    private byte this[
        Vector coordinates]
    {
        get { return tiles[ToArrayIndex(coordinates)]; }
        set { tiles[ToArrayIndex(coordinates)] = value; }
    }

    public int GetDistanceToFarthestLoopPoint()
    {
        var loop = FindLoop();
        return loop.Count / 2;
    }

    public int CalculateLoopInsideArea()
    {
        var loop = FindLoop();

        int area = 0;
        for (int row = 0; row < height; row++)
        {
            int insideCheck = 0;
            for (int column = 0; column < width; column++)
            {
                var location = new Vector(row, column);
                if (loop.Contains(location))
                {
                    insideCheck ^= this[location] & Tile.Pipe;
                    continue;
                }

                if (insideCheck == Tile.Pipe)
                {
                    area++;
                }
            }
        }
        return area;
    }

    public static PipeMap LoadFrom(
        string path)
    {
        var allLines = File.ReadAllLines(path);
        int width = allLines[0].Length;
        int tileCount = allLines.Sum(line => line.Length);
        int height = tileCount / width;

        var tiles = new byte[tileCount];
        int index = 0;
        int startIndex = -1;
        foreach (var line in allLines)
        {
            foreach (var c in line)
            {
                tiles[index] = ToTile(c);
                if (c == 'S')
                {
                    startIndex = index;
                }
                index++;
            }
        }
        return new(
            width, height, tiles, 
            ToCoordinates(startIndex, width));
    }

    private HashSet<Vector> FindLoop()
    {
        if (loop == null)
        {
            var direction = Vector.Null;
            var position = start;
            var tile = this[position];
            var loopLocal = new HashSet<Vector>() { position };
            while (true)
            {
                var back = direction.Reverse;
                for (int i = 0; i < 4; i++)
                {
                    direction = Vector.Directions[i];
                    if (direction == back
                        || (tile & Tile.Masks[i]) == 0)
                    {
                        continue;
                    }

                    position += direction;
                    tile = this[position];
                    break;
                }

                if (position == start)
                {
                    break;
                }
                
                loopLocal.Add(position);
            }

            loop = loopLocal;
        }

        return loop;
    }

    private void FixStart()
    {
        this[start] = (byte)(
            ((GetTileAtWithDotPadding(start + Vector.Up) & Tile.DownMask) << 2)
            | ((GetTileAtWithDotPadding(start + Vector.Right) & Tile.LeftMask) << 2)
            | ((GetTileAtWithDotPadding(start + Vector.Down) & Tile.UpMask) >> 2)
            | ((GetTileAtWithDotPadding(start + Vector.Left) & Tile.RightMask) >> 2));
    }

    private byte GetTileAtWithDotPadding(
        Vector coordinates)
    {
        if (coordinates.Row < 0 || coordinates.Row >= height
            || coordinates.Column < 0 || coordinates.Column >= width)
        {
            return Tile.Dot;
        }

        return this[coordinates];
    }

    private int ToArrayIndex(
        Vector coordinates)
    {
        return coordinates.Row * width + coordinates.Column;
    }

    private static Vector ToCoordinates(
        int arrayIndex, int width)
    {
        return new Vector(
            Row: arrayIndex / width, 
            Column: arrayIndex % width);
    }

    private static byte ToTile(char c)
    {
        return c switch
        {
            '|' => Tile.Pipe,
            '-' => Tile.Dash,
            'L' => Tile.L,
            'J' => Tile.J,
            '7' => Tile.Seven,
            'F' => Tile.F,
            _ => Tile.Dot
        };
    }
}
