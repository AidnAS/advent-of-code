using csharp.utils;

namespace csharp.Puzzles;

public static class Dec01
{
    public static int solvePart1(string? date, bool useTestData = false)
    {
        Console.WriteLine("Day 01, Part 1: Id lists total distance\n");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        // print the data
        // Console.WriteLine("Test Data:");
        // for (int i = 0; i < dfr.Lines.Count; i++) 
        // {
        //     Console.WriteLine(dfr.Lines[i]);
        // }
        // Console.WriteLine("----------");

        // create two lists of Ids
        List<int> firstIds = new List<int>();
        List<int> secondIds = new List<int>();

        CreateLists(lines: dfr.Lines, first: firstIds, second: secondIds);

        // for each line in the file
        // for (int i = 0; i < dfr.Lines.Count; i++)
        // {
        //     // add the first id to one list and the second id to another list
        //     string[] lineIds = dfr.Lines[i].Split(" ", StringSplitOptions.RemoveEmptyEntries);
            
        //     // Console.WriteLine($"lineIds: {lineIds[0]} {lineIds[1]}");

        //     int id1 = Convert.ToInt32(lineIds[0]);
        //     int id2 = Convert.ToInt32(lineIds[1]);

        //     // Console.WriteLine($"id1: {id1}, id2: {id2}");

        //     firstIds.Add(id1);
        //     secondIds.Add(id2);
        // }

        // sort both lists
        firstIds.Sort();
        secondIds.Sort();

        // compare both lists and count
        if (firstIds.Count != secondIds.Count)
        {
            throw new Exception("Error: firstIds and secondIds are not the same length");
        }

        List<int> diffs = new List<int>();

        for (int i = 0; i < firstIds.Count; i++)
        {
            // take absolute difference
            diffs.Add(firstIds[i] >= secondIds[i] ? firstIds[i] - secondIds[i] : secondIds[i] - firstIds[i]);
        }

        // count the total
        int total = 0;

        for (int i = 0; i < diffs.Count; i++)
        {
            total += diffs[i];
        }
        
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {total}\n\n");

        return total;
    }

    public static int solvePart2(string date, bool useTestData = false)
    {
        Console.WriteLine("Day 01, Part 2: Id lists similarity score\n");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        // create two lists of Ids
        List<int> firstIds = new List<int>();
        List<int> secondIds = new List<int>();

        CreateLists(lines: dfr.Lines, first: firstIds, second: secondIds);

        // for each line in the file
        // for (int i = 0; i < dfr.Lines.Count; i++)
        // {
        //     // add the first id to one list and the second id to another list
        //     string[] lineIds = dfr.Lines[i].Split(" ", StringSplitOptions.RemoveEmptyEntries);
            
        //     // Console.WriteLine($"lineIds: {lineIds[0]} {lineIds[1]}");

        //     int id1 = Convert.ToInt32(lineIds[0]);
        //     int id2 = Convert.ToInt32(lineIds[1]);

        //     // Console.WriteLine($"id1: {id1}, id2: {id2}");

        //     firstIds.Add(id1);
        //     secondIds.Add(id2);
        // }

        // compare both lists and count
        if (firstIds.Count != secondIds.Count)
        {
            throw new Exception("Error: firstIds and secondIds are not the same length");
        }

        // find the similarity scores for each id in the first list

        List<int> scores = new List<int>();


        for (int i = 0; i < firstIds.Count; i++)
        {
            var count = 0;

            for (int j = 0; j < secondIds.Count; j++)
            {
                if (firstIds[i] == secondIds[j])
                {
                    count++;
                }
            }
            scores.Add(firstIds[i] * count);
        }

        // count the total
        int total = 0;

        for (int i = 0; i < scores.Count; i++)
        {
            total += scores[i];
        }

        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {total}\n\n");

        return total;

    }


    private static void CreateLists(List<string> lines, List<int> first, List<int> second)
    {
        // for each line in the file
        for (int i = 0; i < lines.Count; i++)
        {
            // add the first id to one list and the second id to another list
            string[] lineIds = lines[i].Split(" ", StringSplitOptions.RemoveEmptyEntries);
            
            // Console.WriteLine($"lineIds: {lineIds[0]} {lineIds[1]}");

            int id1 = Convert.ToInt32(lineIds[0]);
            int id2 = Convert.ToInt32(lineIds[1]);

            // Console.WriteLine($"id1: {id1}, id2: {id2}");

            first.Add(id1);
            second.Add(id2);
        }

    }







}




        // Part 1: pseudo code
        // for each line in the file

        // add the first id to one list and the second id to another list

        // sort both lists

        // compare both lists and count

        // count the total