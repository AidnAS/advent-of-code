using csharp.utils;

namespace csharp.Puzzles;

public static class Dec02
{
    public static int solvePart1(string? date, bool useTestData = false)
    {
        Console.WriteLine("Day 02, Part 1: Safe reports\n");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        int totalSafeReports = 0;

        // Console.WriteLine($"Total number of reports: {dfr.Lines.Count}\n");

        // for each report (line)
        for (int i = 0; i < dfr.Lines.Count; i++) 
        {
            // split the report to get the levels
            string[] report = dfr.Lines[i].Split(" ", StringSplitOptions.RemoveEmptyEntries);

            if (reportIsSafe(report))
            {
                totalSafeReports++;
            }
        }

        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {totalSafeReports}\n");

        return totalSafeReports;
    }

    public static int solvePart2(string date, bool useTestData = false)
    {
        // log current puzzle
        Console.WriteLine("Day 02, Part 2: Problem Dampener\n");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        int totalSafeReports = 0;

        // Console.WriteLine($"Total number of reports: {dfr.Lines.Count}\n");

        // for each report (line)
        for (int i = 0; i < dfr.Lines.Count; i++) 
        {
            // split the report to get the levels
            string[] report = dfr.Lines[i].Split(" ", StringSplitOptions.RemoveEmptyEntries);

            if (reportIsSafePt2(report))
            {
                totalSafeReports++;
            }
        }

        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {totalSafeReports}");

        return totalSafeReports;
    }

    private static bool reportIsSafe(string[] report)
    {
        if (IsGraduallyIncreasing(report) || IsGraduallyDecreasing(report))
        {
            return true;
        }
        else
        {
            return false;
        }
    }
    
    private static bool reportIsSafePt2(string[] report)
    {
        // if the report is always increasing or decreasing and within tolerance, return true
        if (IsGraduallyIncreasing(report) || IsGraduallyDecreasing(report))
        {
            return true;
        }
        else if (safeWithBadValue(report))
        {
            return true;
        }
        else
        {
            return false;
        }
    }

    private static bool safeWithBadValue(string[] report)
    {
        for (int i = 0; i < report.Length; i++)
        {
            // try removing every value and see if the report is safe
            List<string> reportList = new List<string>(report);
            reportList.RemoveAt(i);
            var newReport = reportList.ToArray();
            if (IsGraduallyIncreasing(newReport) || IsGraduallyDecreasing(newReport))
            {
                return true;
            }
        }
        return false;
    }

    private static bool IsGraduallyIncreasing(string[] report)
    {
        for (int i = 0; i < report.Length - 1; i++)
        {
            var currentLevel = Convert.ToInt32(report[i]);
            var nextLevel = Convert.ToInt32(report[i+1]);
            if (currentLevel < nextLevel && IsWithinTolerance(currentLevel, nextLevel))
            {
                continue;
            }
            else
            {
                return false;
            }
        }
        return true;
    }

    private static bool IsGraduallyDecreasing(string[] report)
    {
        for (int i = 0; i < report.Length - 1; i++)
        {
            var currentLevel = Convert.ToInt32(report[i]);
            var nextLevel = Convert.ToInt32(report[i+1]);
            if (currentLevel > nextLevel && IsWithinTolerance(currentLevel, nextLevel))
            {
                continue;
            }
            else
            {
                return false;
            }
        }
        return true;
    }

    private static bool IsWithinTolerance(int currentLevel, int nextLevel)
    {
        var absDelta = Math.Abs(currentLevel - nextLevel);
        return absDelta >= 1 && absDelta <= 3;
    }
    
}
