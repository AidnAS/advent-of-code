using System.Text;
using System.Text.RegularExpressions;

namespace Aoc2024;

using Position = (int row, int col);

public static class Problem10
{
    public static int Part1()
    {
        var data = File.ReadAllText("data/problem10.txt");
        var lines = data.Split("\n", StringSplitOptions.RemoveEmptyEntries);
        var numRows = lines.Length;
        var numCols = lines[0].Length;
        var adjacencyGraph = new Dictionary<Position, Position[]>();
        var startPositions = new List<Position>();
        for (int i = 0; i < numRows; i++)
        {
            for (int j = 0; j < numCols; j++)
            {
                Position currentPosition = (i, j);
                var currentHeight = lines[i][j];
                Position[] adjacentPositions = [(i - 1, j), (i, j - 1), (i + 1, j), (i, j + 1)];
                var reachable = adjacentPositions.Where(IsWithinBounds)
                    .Where(pos => lines[pos.row][pos.col] != '.')
                    .Where(pos => lines[pos.row][pos.col] - currentHeight == 1).ToArray();
                adjacencyGraph.Add(currentPosition, reachable);
                if (currentHeight == '0')
                {
                    startPositions.Add(currentPosition);
                }
            }
        }


        int sum = 0;

        foreach (var start in startPositions)
        {
            List<Position> nodesToVisit = [start];
            var visitedNodes = new HashSet<Position>();
            while (nodesToVisit.Count > 0)
            {
                var currentNode = nodesToVisit[^1];
                nodesToVisit.RemoveAt(nodesToVisit.Count - 1);
                if (!visitedNodes.Add(currentNode))
                {
                    continue;
                }

                var height = lines[currentNode.row][currentNode.col];
                if (height == '9')
                {
                    sum++;
                }

                nodesToVisit.AddRange(adjacencyGraph[currentNode].Where(pos => !visitedNodes.Contains(pos)).ToArray());
            }
        }

        return sum;

        bool IsWithinBounds(Position pos) =>
            pos switch
            {
                (< 0, _) => false,
                (_, < 0) => false,
                var (row, col) when row >= numRows || col >= numCols => false,
                _ => true
            };
    }

    public static int Part2()
    {
        var data = File.ReadAllText("data/problem10.txt");
        var lines = data.Split("\n", StringSplitOptions.RemoveEmptyEntries);
        var numRows = lines.Length;
        var numCols = lines[0].Length;
        var adjacencyGraph = new Dictionary<Position, Position[]>();
        var startPositions = new List<Position>();
        for (int i = 0; i < numRows; i++)
        {
            for (int j = 0; j < numCols; j++)
            {
                Position currentPosition = (i, j);
                var currentHeight = lines[i][j];
                Position[] adjacentPositions = [(i - 1, j), (i, j - 1), (i + 1, j), (i, j + 1)];
                var reachable = adjacentPositions.Where(IsWithinBounds)
                    .Where(pos => lines[pos.row][pos.col] != '.')
                    .Where(pos => lines[pos.row][pos.col] - currentHeight == 1).ToArray();
                adjacencyGraph.Add(currentPosition, reachable);
                if (currentHeight == '0')
                {
                    startPositions.Add(currentPosition);
                }
            }
        }


        int sum = 0;

        foreach (var start in startPositions)
        {
            List<Position> nodesToVisit = [start];
            while (nodesToVisit.Count > 0)
            {
                var currentNode = nodesToVisit[^1];
                nodesToVisit.RemoveAt(nodesToVisit.Count - 1);

                var height = lines[currentNode.row][currentNode.col];
                if (height == '9')
                {
                    sum++;
                }

                nodesToVisit.AddRange(adjacencyGraph[currentNode]);
            }
        }

        return sum;

        bool IsWithinBounds(Position pos) =>
            pos switch
            {
                (< 0, _) => false,
                (_, < 0) => false,
                var (row, col) when row >= numRows || col >= numCols => false,
                _ => true
            };
    }
}