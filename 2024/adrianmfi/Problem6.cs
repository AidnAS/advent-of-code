using System.Text;
using System.Text.RegularExpressions;

namespace Aoc2024;

public static class Problem6
{
    public static int Part1()
    {
        var data = File.ReadAllText("data/problem6.txt");

        var lines = data.Split("\n", StringSplitOptions.RemoveEmptyEntries);
        var numRows = lines.Length;
        var numCols = lines[0].Length;

        var inRow = new Dictionary<int, List<int>>();
        var inCol = new Dictionary<int, List<int>>();
        for (int i = 0; i < lines.Length; i++)

        {
            var line = lines[i];
            var matches = Regex.Matches(line, "#");
            foreach (Match match in matches)
            {
                if (!inCol.TryAdd(match.Index, [i]))
                {
                    inCol[match.Index].Add(i);
                }
            }

            inRow[i] = matches.Select(match => match.Index).ToList();
        }

        foreach (var col in inCol)
        {
            col.Value.Sort();
        }

        var startPosition = FindStartPosition(lines);

        var currentPosition = startPosition;
        var direction = Direction.Up;

        var covered = new bool[numRows, numCols];
        while (true)
        {
            if (direction == Direction.Up)
            {
                var nextIndex = inCol[currentPosition.col].FindLastIndex(row => row <= currentPosition.row);
                var next = nextIndex == -1 ? 0 : inCol[currentPosition.col][nextIndex] + 1;
                for (var i = next; i <= currentPosition.row; i++)
                {
                    covered[i, currentPosition.col] = true;
                }

                if (nextIndex == -1)
                {
                    break;
                }

                currentPosition = (next, currentPosition.col);
                direction = Direction.Right;
            }
            else if (direction == Direction.Right)
            {
                var nextIndex = inRow[currentPosition.row].FindIndex(col => col >= currentPosition.col);
                var next = nextIndex == -1 ? numCols - 1 : inRow[currentPosition.row][nextIndex] - 1;
                for (var i = currentPosition.col; i <= next; i++)
                {
                    covered[currentPosition.row, i] = true;
                }

                if (nextIndex == -1)
                {
                    break;
                }

                currentPosition = (currentPosition.row, next);
                direction = Direction.Down;
            }
            else if (direction == Direction.Down)
            {
                var nextIndex = inCol[currentPosition.col].FindIndex(row => row >= currentPosition.row);
                var next = nextIndex == -1 ? numRows - 1 : inCol[currentPosition.col][nextIndex] - 1;
                for (var i = currentPosition.row; i <= next; i++)
                {
                    covered[i, currentPosition.col] = true;
                }

                if (nextIndex == -1)
                {
                    break;
                }

                currentPosition = (next, currentPosition.col);
                direction = Direction.Left;
            }
            else if (direction == Direction.Left)
            {
                var nextIndex = inRow[currentPosition.row].FindLastIndex(col => col <= currentPosition.col);
                var next = nextIndex == -1 ? 0 : inRow[currentPosition.row][nextIndex] + 1;
                for (var i = next; i <= currentPosition.col; i++)
                {
                    covered[currentPosition.row, i] = true;
                }

                if (nextIndex == -1)
                {
                    break;
                }

                currentPosition = (currentPosition.row, next);
                direction = Direction.Up;
            }
        }


        return covered.Cast<bool>().Count(value => value);
    }


    public static int Part2()
    {
        var data = File.ReadAllText("data/problem6.txt");

        var lines = data.Split("\n", StringSplitOptions.RemoveEmptyEntries);
        var numRows = lines.Length;
        var numCols = lines[0].Length;

        var inRow = new Dictionary<int, List<int>>();
        var inCol = new Dictionary<int, List<int>>();
        for (int i = 0; i < lines.Length; i++)
        {
            var line = lines[i];
            var matches = Regex.Matches(line, "#");
            foreach (Match match in matches)
            {
                if (!inCol.TryAdd(match.Index, [i]))
                {
                    inCol[match.Index].Add(i);
                }
            }

            inRow[i] = matches.Select(match => match.Index).ToList();
        }

        foreach (var col in inCol)
        {
            col.Value.Sort();
        }

        var startPosition = FindStartPosition(lines);

        int numLoops = 0;

        for (var targetRow = 0; targetRow < numRows; targetRow++)
        {
            for (var targetCol = 0; targetCol < numCols; targetCol++)
            {
                if ((targetRow, targetCol) == startPosition)
                {
                    continue;
                }

                if (inRow[targetRow].Contains(targetCol))
                {
                    continue;
                }

                var modifiedInRow = inRow.ToDictionary(entry => entry.Key, entry => new List<int>(entry.Value));
                if (modifiedInRow.TryGetValue(targetRow, out var rowList))
                {
                    rowList.Add(targetCol);
                    rowList.Sort();
                }
                else
                {
                    modifiedInRow[targetRow] = new List<int> { targetCol };
                }

                var modifiedInCol = inCol.ToDictionary(entry => entry.Key, entry => new List<int>(entry.Value));
                if (modifiedInCol.TryGetValue(targetCol, out var colList))
                {
                    colList.Add(targetRow);
                    colList.Sort();
                }
                else
                {
                    modifiedInCol[targetCol] = new List<int> { targetRow };
                }

                var currentPosition = startPosition;
                var direction = Direction.Up;
                var visited = new HashSet<(int row, int col, Direction direction)>();
                while (true)
                {
                    if (!visited.Add((currentPosition.row, currentPosition.col, direction)))
                    {
                        numLoops++;
                        break;
                    }

                    if (direction == Direction.Up)
                    {
                        var nextIndex = modifiedInCol.ContainsKey(currentPosition.col) ? modifiedInCol[currentPosition.col].FindLastIndex(row => row <= currentPosition.row) : -1;
                        var next = nextIndex == -1 ? 0 : modifiedInCol[currentPosition.col][nextIndex] + 1;

                        if (nextIndex == -1)
                        {
                            break;
                        }

                        currentPosition = (next, currentPosition.col);
                        direction = Direction.Right;
                    }
                    else if (direction == Direction.Right)
                    {
                        var nextIndex = modifiedInRow.ContainsKey(currentPosition.row) ? modifiedInRow[currentPosition.row].FindIndex(col => col >= currentPosition.col) : -1;
                        var next = nextIndex == -1 ? numCols - 1 : modifiedInRow[currentPosition.row][nextIndex] - 1;

                        if (nextIndex == -1)
                        {
                            break;
                        }

                        currentPosition = (currentPosition.row, next);
                        direction = Direction.Down;
                    }
                    else if (direction == Direction.Down)
                    {
                        var nextIndex = modifiedInCol.ContainsKey(currentPosition.col) ? modifiedInCol[currentPosition.col].FindIndex(row => row >= currentPosition.row) : -1;
                        var next = nextIndex == -1 ? numRows - 1 : modifiedInCol[currentPosition.col][nextIndex] - 1;

                        if (nextIndex == -1)
                        {
                            break;
                        }

                        currentPosition = (next, currentPosition.col);
                        direction = Direction.Left;
                    }
                    else if (direction == Direction.Left)
                    {
                        var nextIndex = modifiedInRow.ContainsKey(currentPosition.row) ? modifiedInRow[currentPosition.row].FindLastIndex(col => col <= currentPosition.col) : -1;
                        var next = nextIndex == -1 ? 0 : modifiedInRow[currentPosition.row][nextIndex] + 1;

                        if (nextIndex == -1)
                        {
                            break;
                        }

                        currentPosition = (currentPosition.row, next);
                        direction = Direction.Up;
                    }
                }
            }
        }


        return numLoops;
    }


    enum Direction
    {
        Up,
        Down,
        Left,
        Right
    }


    private static (int row, int col) FindStartPosition(string[] lines)
    {
        for (int i = 0; i < lines.Length; i++)
        {
            var line = lines[i];
            for (int j = 0; j < line.Length; j++)
            {
                if (line[j] == '^')
                {
                    return (i, j);
                }
            }
        }

        throw new Exception("No start position found");
    }
}