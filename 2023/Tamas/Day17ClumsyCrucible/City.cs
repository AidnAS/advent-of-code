namespace Day17ClumsyCrucible;

internal sealed class City
{
    private readonly byte[] grid;
    private readonly int width;
    private readonly int height;

    private City(
        byte[] grid,
        int width,
        int height)
    {
        this.grid = grid;
        this.width = width;
        this.height = height;
    }

    public int CalculateMinHeatLoss(
        int minDistanceInOneDirection,
        int maxDistanceInOneDiretion)
    {
        return CalculateMinHeatLoss(
            from: Vector.Null, 
            to: new Vector(height - 1, width - 1),
            minDistanceInOneDirection,
            maxDistanceInOneDiretion);
    }

    private int CalculateMinHeatLoss(
        Vector from,
        Vector to,
        int minDistanceInOneDirection,
        int maxDistanceInOneDiretion)
    {
        var visitedMap = new VisitRecord[width * height];
        var queue = new PriorityQueue<Cursor, int>();
        queue.Enqueue(
            new Cursor(from, LastDirection: default, StraightCount: 0), 
            priority: 0);
        while (queue.TryDequeue(out var cursor, out int heatLoss))
        {
            var (location, lastDirection, straightCount) = cursor;
            if (location == to)
            {
                return heatLoss;
            }

            var backDirection = Mirror(lastDirection);
            for (int directionAsNumber = 0; directionAsNumber < 4; directionAsNumber++)
            {
                var direction = (Direction)directionAsNumber;
                if (direction == backDirection && straightCount != 0)
                {
                    continue;
                }

                int straightCountInDirection = direction == lastDirection ? straightCount : 0;
                int stepCount = Math.Max(minDistanceInOneDirection - 1 - straightCountInDirection, 0) + 1;
                int nextStraightCount = straightCountInDirection + stepCount;
                if (nextStraightCount > maxDistanceInOneDiretion)
                {
                    continue;
                }

                var directionVector = Vector.Directions[directionAsNumber];
                if (!IsInGrid(location + directionVector * stepCount))
                {
                    continue;
                }

                var next = location;
                int nextIndex = -1;
                int nextHeatLoss = heatLoss;
                for (int i = 0; i < stepCount; i++)
                {
                    next += directionVector;
                    nextIndex = ToIndex(next);
                    nextHeatLoss += grid[nextIndex];
                }

                var visitRecord = visitedMap[nextIndex];
                if (!visitRecord.NeedsVisit(direction, nextStraightCount, minDistanceInOneDirection))
                {
                    continue;
                }

                queue.Enqueue(
                    new Cursor(next, direction, nextStraightCount),
                    nextHeatLoss);
                visitedMap[nextIndex] = visitRecord.Set(direction, nextStraightCount);
            }
        }
        return int.MaxValue;
    }

    public static City Load(
        string path)
    {
        var allLines = File.ReadAllLines(path);
        int tileCount = allLines.Sum(line => line.Length);
        var grid = new byte[tileCount];
        int index = 0;
        foreach (var line in allLines)
        {
            foreach (var c in line)
            {
                grid[index++] = (byte)(c - '0');
            }
        }

        int width = allLines[0].Length;
        int height = tileCount / width;
        return new City(grid, width, height);
    }

    private bool IsInGrid(
        Vector location)
    {
        return location.Row >= 0 && location.Column >= 0
            && location.Row < height && location.Column < width;
    }

    private int ToIndex(
        Vector location)
    {
        return location.Row * width + location.Column;
    }

    private static Direction Mirror(
        Direction direction)
    {
        int asNumber = (int)direction;
        asNumber = (asNumber + 2) % 4;
        return (Direction)asNumber;
    }

    private readonly record struct Cursor(
        Vector Location,
        Direction LastDirection,
        int StraightCount);

    private readonly struct VisitRecord
    {
        public static readonly VisitRecord Empty = new();

        private const uint FullShortMask = 0xFFFF;

        private readonly ulong data;

        private VisitRecord(
            ulong data)
        {
            this.data = data;
        }

        public bool NeedsVisit(
            Direction direction,
            int value,
            int minDistanceInOneDirection)
        {
            int directionShift = GetDirectionShift(direction);
            ushort existingValue = (ushort)((data >> directionShift) & FullShortMask);
            return minDistanceInOneDirection == 1
                ? ((existingValue << (16 - value)) & 0xFFFF) == 0
                : (existingValue & (ushort)(1 << (value - 1))) == 0;
        }

        public VisitRecord Set(
            Direction direction,
            int value)
        {
            int directionShift = GetDirectionShift(direction);
            ulong valueAsFlag = (ulong)(1 << (value - 1));
            return new(data | (valueAsFlag << directionShift));
        }

        private static int GetDirectionShift(
            Direction direction)
        {
            return ((int)direction) * 16;
        }
    }
}
