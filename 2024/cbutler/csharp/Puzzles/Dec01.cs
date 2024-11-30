using csharp.utils;

namespace csharp.Puzzles;

public static class Dec01
{
    public static void solvePart1(string? date, bool useTestData = false)
    {
        Console.WriteLine("Day 01, Part 1");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        for (int i = 0; i < dfr.Lines.Count; i++) 
        {
            Console.WriteLine(dfr.Lines[i]);
        }
        
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {dfr.Lines.Count}");
    }

    public static void solvePart2(string date, bool useTestData = false)
    {
        Console.WriteLine("Day 01, Part 1");
    }
}
