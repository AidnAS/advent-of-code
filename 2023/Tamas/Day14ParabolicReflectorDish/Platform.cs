using System.Collections;
using System.Text;

namespace Day14ParabolicReflectorDish;

internal sealed class Platform
{
    private const ulong SnapshotInterval = 1000;

    private readonly Tile[] grid;
    private readonly int width;
    private readonly int height;
    
    private readonly View[] views;

    private readonly List<BitArray> snapshotList = new();
    private readonly MultiMapLookup<ulong, ulong> fingerprintLookup = new();

    private Platform(
        Tile[] grid,
        int width,
        int height)
    {
        this.grid = grid;
        this.width = width;
        this.height = height;

        views = new View[]
        {
            new View(this, 1, 0),
            new View(this, 0, -1),
            new View(this, -1, 0),
            new View(this, 0, 1)
        };
    }

    public void PerformManyCycles(
        ulong count)
    {
        var (startIndex, length) = FindLoop();

        if (count >= startIndex)
        {
            count = startIndex + ((count - startIndex) % length);
        }

        Seek(count);
    }

    public int GetLoadNorth()
    {
        return GetLoadNorthAndWest().NorthLoad;
    }

    public void Tilt(
        Direction direction)
    {
        var view = views[(int)direction];
        for (int column = 0; column < view.Width; column++)
        {
            int rollTargetRow = -1;
            for (int row = 0; row < view.Height; row++)
            {
                var tile = view[row, column];
                if (tile == Tile.RoundedStone)
                {
                    if (rollTargetRow != -1)
                    {
                        view[rollTargetRow, column] = Tile.RoundedStone;
                        view[row, column] = Tile.EmptySpace;
                        rollTargetRow++;
                    }
                }
                else if (tile == Tile.CubeShapedStone)
                {
                    rollTargetRow = -1;
                }
                else if (tile == Tile.EmptySpace)
                {
                    if (rollTargetRow == -1)
                    {
                        rollTargetRow = row;
                    }
                }
            }
        }
    }

    public static Platform Load(
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
        return new Platform(grid, width, height);
                
        static Tile ToTile(
            char c)
        {
            return c switch
            {
                'O' => Tile.RoundedStone,
                '#' => Tile.CubeShapedStone,
                _ => Tile.EmptySpace
            };
        }
    }

    public override string ToString()
    {
        int tileCount = width * height;
        var builder = new StringBuilder(tileCount);
        for (int i = 0; i < tileCount; i++)
        {
            builder.Append(ToChar(grid[i]));
            if (i % width == width - 1)
            {
                builder.AppendLine();
            }
        }
        return builder.ToString();

        static char ToChar(Tile tile)
            => tile switch
            {
                Tile.EmptySpace => '.',
                Tile.RoundedStone => 'O',
                _ => '#'
            };
    }

    private (ulong StartIndex, ulong Length) FindLoop()
    {
        snapshotList.Clear();
        fingerprintLookup.Clear();

        ulong nextSnapshotIndex = 0;
        for (ulong index = 0; true; index++)
        {
            if (index == nextSnapshotIndex)
            {
                snapshotList.Add(Save());
                nextSnapshotIndex += SnapshotInterval;
            }

            var fingerprint = GetFingerprint();
            BitArray? current = null;
            foreach (var matchingIndex in fingerprintLookup[fingerprint])
            {
                current ??= Save();
                Seek(matchingIndex);
                
                if (BitArrayEqualityComparer.Default.Equals(current, Save()))
                {
                    return (matchingIndex, index - matchingIndex);
                }
            }
            if (current != null)
            {
                Load(current);
            }
            fingerprintLookup.Add(fingerprint, index);

            PerformOneCycle();
        }
    }

    private void Seek(
        ulong index)
    {
        var (snapshotIndex, cyclesAfterSnapshot) = ulong.DivRem(index, SnapshotInterval);
        var snapshot = snapshotList[(int)snapshotIndex];
        Load(snapshot);
        for (int i = 0; i < (int)cyclesAfterSnapshot; i++)
        {
            PerformOneCycle();
        }
    }

    private void PerformOneCycle()
    {
        Tilt(Direction.North);
        Tilt(Direction.West);
        Tilt(Direction.South);
        Tilt(Direction.East);
    }

    private (int NorthLoad, int WestLoad) GetLoadNorthAndWest()
    {
        var view = views[(int)Direction.North];
        int northLoad = 0;
        int westLoad = 0;
        for (int column = 0; column < view.Width; column++)
        {
            for (int row = 0; row < view.Height; row++)
            {
                var tile = view[row, column];
                if (tile != Tile.RoundedStone)
                {
                    continue;
                }
                northLoad += view.Height - row;
                westLoad += view.Width - column;
            }
        }
        return (northLoad, westLoad);
    }

    private ulong GetFingerprint()
    {
        var (northLoad, westLoad) = GetLoadNorthAndWest();
        return (((ulong)northLoad) << 32) + (ulong)westLoad;
    }

    private BitArray Save()
    {
        var bits = new BitArray(grid.Length);
        for (int i = 0; i < grid.Length; i++)
        {
            if (grid[i] == Tile.RoundedStone)
            {
                bits[i] = true;
            }
        }
        return bits;
    }

    private void Load(
        BitArray bits)
    {
        for (int i = 0; i < grid.Length; i++)
        {
            if (grid[i] == Tile.CubeShapedStone)
            {
                continue;
            }
            grid[i] = bits[i]
                ? Tile.RoundedStone
                : Tile.EmptySpace;
        }
    }

    private sealed class View
    {
        private readonly Platform platform;
        private readonly int m1, m2, p1, p2;

        public View(
            Platform platform,
            int m1,
            int m2)
        {
            this.platform = platform;
            this.m1 = m1;
            this.m2 = m2;

            (Height, Width) = m1 != 0 
                ? (platform.height, platform.width) 
                : (platform.width, platform.height);

            (p1, p2) = (m1, m2) switch
            {
                (-1, _) => (platform.height - 1, platform.width - 1),
                (_, -1) => (platform.width - 1, platform.height - 1),
                _ => (0, 0)
            };
        }

        public int Height { get; }
        public int Width { get; }

        public Tile this[int row, int column]
        {
            get { return platform.grid[ToIndex(row, column)]; }
            set { platform.grid[ToIndex(row, column)] = value; }
        }

        private int ToIndex(int row, int column)
        {
            return (m1 * row + m2 * column + p1) * Width
                + m2 * row + m1 * column + p2;
        }
    }
}
