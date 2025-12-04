

namespace adrianmfi;

public static class Day4
{
    static readonly string test = """
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.
    """;
    public static int Part1()
    {
        // var lines = test.Split('\n');
        var lines = File.ReadAllLines("inputs/day4.txt");
        var grid = new Grid(lines);
        int result = 0;
        for (int i = 0; i < grid.rows; i++)
        {
            for (int j = 0; j < grid.cols; j++)
            {
                if (grid.Test(i, j))
                {
                    result++;
                }
            }
        }

        return result;

    }

    public static int Part2()
    {
        // var lines = test.Split('\n');
        var lines = File.ReadAllLines("inputs/day4.txt");
        var grid = new Grid(lines);
        int result = 0;
        var resultChanged = true;
        while (resultChanged)
        {
            resultChanged = false;
            for (int i = 0; i < grid.rows; i++)
            {
                for (int j = 0; j < grid.cols; j++)
                {
                    if (grid.Test(i, j))
                    {
                        grid.Remove(i, j);
                        result++;
                        resultChanged = true;
                    }
                }
            }
        }

        return result;

    }


}

class Grid
{
    public readonly bool[,] grid;
    public readonly int rows;
    public readonly int cols;
    public Grid(string[] gridStrings)
    {
        rows = gridStrings.Length;
        cols = gridStrings[0].Length;
        grid = new bool[rows, cols];
        for (var i = 0; i < rows; i++)
        {
            for (var j = 0; j < cols; j++)
            {
                grid[i, j] = gridStrings[i][j] == '@';
            }
        }
    }

    public bool Test(int row, int col)
    {
        if (!grid[row, col]) { return false; }
        (int row, int col)[] positions = [
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col+1),
            (row, col-1),
            (row, col+1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col+1),
        ];
        return positions.Sum(p => GetValueSafe(p.row, p.col) ? 1 : 0) < 4;
    }

    private bool GetValueSafe(int row, int col)
    {
        if (row < 0 || row >= rows || col < 0 || col >= cols)
        {
            return false;
        }
        return grid[row, col];
    }


    public void Remove(int row, int col)
    {
        grid[row, col] = false;
    }

};
