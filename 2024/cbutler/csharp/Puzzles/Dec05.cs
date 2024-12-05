using csharp.utils;

namespace csharp.Puzzles;


public static class Dec05
{
    public static int solvePart1(string? date, bool useTestData = false)
    {
        // log current puzzle
        Console.WriteLine("Day 05, Part 1: \n");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        for (int i = 0; i < dfr.Lines.Count; i++) 
        {
            Console.WriteLine(dfr.Lines[i]);
        }
        
        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {dfr.Lines.Count}\n");

        return 0;
    }

    public static int solvePart2(string date, bool useTestData = false)
    {
        // log current puzzle
        Console.WriteLine("Day 05, Part 2: \n");

        // perform magic here

        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: 0\n");

        return 0;
    }
}
