using csharp.utils;

namespace csharp.Puzzles;

// This is the boilerplate code for each day's puzzle.
public static class Boilerplate
{
    public static void solvePart1(string? date, bool useTestData = false)
    {
        // log current puzzle
        Console.WriteLine("Day XX, Part 1");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        for (int i = 0; i < dfr.Lines.Count; i++) 
        {
            Console.WriteLine(dfr.Lines[i]);
        }
        
        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {dfr.Lines.Count}");
    }

    public static void solvePart2(string date, bool useTestData = false)
    {
        // log current puzzle
        Console.WriteLine("Day XX, Part 1");

        // perform magic here

        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: 0");
    }
}
