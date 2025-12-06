

using System.ComponentModel;
using System.Text;

namespace adrianmfi;

public static class Day6
{
    static readonly string test = """
    123 328  51 64 
     45 64  387 23 
      6 98  215 314
    *   +   *   +  
    """;
    public static long Part1()
    {
        List<Operation> operations = [];
        List<List<long>> numbersRowCol = [];
        // foreach (var line in test.Split('\n'))
        foreach (var line in File.ReadLines("inputs/day6.txt"))
        {
            var splitLine = line.Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);
            if (splitLine[0] == "*" || splitLine[0] == "+")
            {
                operations = [.. splitLine.Select(ParseOperation)];
            }
            else
            {
                numbersRowCol.Add([.. splitLine.Select(long.Parse)]);
            }
        }
        List<Problem> problems = [];
        for (int i = 0; i < numbersRowCol[0].Count; i++)
        {
            List<long> numbersInCol = [];
            for (int j = 0; j < numbersRowCol.Count; j++)
            {
                numbersInCol.Add(numbersRowCol[j][i]);
            }
            problems.Add(new([.. numbersInCol], operations[i]));
        }


        return problems.Sum(problem => problem.CalculateAnswer());

    }


    public static long Part2()
    {
        List<Operation> operations = [];
        List<List<char>> chars = [];

        foreach (var line in File.ReadLines("inputs/day6.txt"))
        // foreach (var line in test.Split('\n'))
        {
            if (line[0] == '*' || line[0] == '+')
            {
                var splitLine = line.Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries);
                operations = [.. splitLine.Select(ParseOperation)];
            }
            else
            {
                chars.Add([.. line.ToCharArray()]);
            }
        }

        List<Problem> problems = [];
        int operationIndex = 0;
        List<string> rowStrings = [];
        for (int col = 0; col < chars[0].Count; col++)
        {
            StringBuilder rowStringBuilder = new();
            for (int row = 0; row < chars.Count; row++)
            {
                rowStringBuilder.Append(chars[row][col]);
            }
            var rowString = rowStringBuilder.ToString();
            if (rowString.IsWhiteSpace())
            {
                problems.Add(new([.. rowStrings.Select(long.Parse)], operations[operationIndex]));
                rowStrings = [];
                operationIndex++;
            }
            else
            {
                rowStrings.Add(rowString);
            }
        }
        problems.Add(new([.. rowStrings.Select(long.Parse)], operations[operationIndex]));

        return problems.Sum(problem => problem.CalculateAnswer());

    }


    enum Operation
    {
        Multiply,
        Add
    };
    static Operation ParseOperation(string input) => input switch
    {
        "*" => Operation.Multiply,
        "+" => Operation.Add,
        _ => throw new ArgumentException("WRONG!"),
    };


    record Problem(long[] Numbers, Operation Operation)
    {
        public long CalculateAnswer()
        {
            return Operation switch
            {
                Operation.Multiply => Numbers.Aggregate((aggregate, total) => aggregate * total),
                Operation.Add => Numbers.Sum(),
                _ => throw new NotImplementedException()
            };
        }

    }

}
