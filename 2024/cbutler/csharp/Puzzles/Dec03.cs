using csharp.utils;

namespace csharp.Puzzles;

public static class Dec03
{
    public static int solvePart1(string? date, bool useTestData = false)
    {
        // log current puzzle
        Console.WriteLine("Day 03, Part 1: Muls\n");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        var totalMuls = new List<Tuple<int, int>>();

        for (int i = 0; i < dfr.Lines.Count; i++) 
        {
            Console.WriteLine(dfr.Lines[i]);
            var indicies = GetAllIndicies(dfr.Lines[i], "mul");
            
            totalMuls.AddRange(GetAllMuls(dfr.Lines[i], indicies));
        }

        var total = ComputeMuls(totalMuls);

        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {total}\n");

        return total;
    }

    public static int solvePart2(string date, bool useTestData = false)
    {
        // log current puzzle
        Console.WriteLine("Day 03, Part 2: Do's and Dont's\n");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 2);
        // dfr.ReadFile();
        // this took me far too long to understand that you should not read the file line by line but rather as one string.
        dfr.ReadFileAsSingleLine(); 

        var totalMuls = new List<Tuple<int, int>>();

        for (int i = 0; i < dfr.Lines.Count; i++) 
        {
            var indicies = GetAllIndicies(dfr.Lines[i], "mul");   
            totalMuls.AddRange(GetAllPt2Muls(dfr.Lines[i], indicies));
        }

        var total = ComputeMuls(totalMuls);

        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {total}\n");

        return total;
    }

    private static List<int> GetAllIndicies(string str, string sub)
    {
        if (String.IsNullOrEmpty(str) || String.IsNullOrEmpty(sub))
        {
            throw new ArgumentException("String or substring is null or empty.");
        }

        List<int> indicies = new List<int>();
        for (int i = 0; i < str.Length; i++)
        {
            var index = str.IndexOf(sub, i);
            if (index != -1)
            {
                indicies.Add(index);
                i = index;
            }
        }
        return indicies;
    }

    private static List<Tuple<int, int>> GetAllMuls(string str, List<int> indicies)
    {
        if (String.IsNullOrEmpty(str) || indicies.Count == 0)
        {
            throw new ArgumentException("String or indicies is null or empty.");
        }

        List<Tuple<int, int>> muls = new List<Tuple<int, int>>();

        var maxDistance = 11; // mul(num,num) = 11

        // for each index, if they can be parsed correctly, extract the numbers....  
        // there cannot be any whitespace
        // mul(num,num) 
        for (int i = 0; i < indicies.Count; i++)
        {
            int val1Idx = i+4; // mul(
            // Console.WriteLine($"val1Idx: {val1Idx}");
            int endBracket = str.IndexOf(')', indicies[i]);
            // Console.WriteLine($"endBracket: {endBracket}");

            if (endBracket != -1  && endBracket - indicies[i] <= maxDistance)
            {
                // var currentMul = str.Substring(indicies[i], maxDistance);
                // Console.WriteLine($"currentMul: {currentMul}");

                var val1Start = str.IndexOf('(', indicies[i]) + 1;
                var val1Len = str.IndexOf(',', indicies[i]) - val1Start;
                if (val1Start < 0 || val1Len < 0)
                {
                    continue;
                    // throw new ArgumentException("val1Start or val1Len is -1.");
                }
                var val1 = str.Substring(val1Start, val1Len);
                // Console.WriteLine($"val1: {val1}");

                var val2Start = str.IndexOf(',', indicies[i]) + 1;
                var val2Len = str.IndexOf(')', indicies[i]) - val2Start;
                if (val2Start < 0 || val2Len < 0)
                {
                    continue;
                    // throw new ArgumentException("val2Start or val2Len is -1.");
                }
                var val2 = str.Substring(val2Start, val2Len);
                // Console.WriteLine($"val2: {val2}");

                muls.Add(new Tuple<int, int>(int.Parse(val1), int.Parse(val2)));
            }
            // Console.WriteLine($"index: {indicies[i]}");
        }
        return muls;
    }

    private static List<Tuple<int, int>> GetAllPt2Muls(string str, List<int> indicies)
    {
        if (String.IsNullOrEmpty(str) || indicies.Count == 0)
        {
            throw new ArgumentException("String or indicies is null or empty.");
        }

        List<Tuple<int, int>> muls = new List<Tuple<int, int>>();

        var maxDistance = 11; // mul(num,num) = 11

        // for each index, if they can be parsed correctly, extract the numbers....  
        // there cannot be any whitespace
        // mul(num,num) 
        for (int i = 0; i < indicies.Count; i++)
        {
            // Console.WriteLine($"val1Idx: {val1Idx}");
            int endBracket = str.IndexOf(')', indicies[i]);
            // Console.WriteLine($"endBracket: {endBracket}");

            if (endBracket != -1  && endBracket - indicies[i] <= maxDistance)
            {
                // if (str.Length > indicies[i] + maxDistance)
                // {
                //     var currentMul = str.Substring(indicies[i], maxDistance);
                //     Console.WriteLine($"currentMul: {currentMul}");
                // }
                // else 
                // {
                //     var currentMul = str.Substring(indicies[i], str.Length - indicies[i]);
                //     Console.WriteLine($"currentMul: {currentMul}");
                // }
                
                var val1Start = str.IndexOf('(', indicies[i]) + 1;
                var val1Len = str.IndexOf(',', indicies[i]) - val1Start;
                if (val1Start < 0 || val1Len <= 0)
                {
                    continue;
                    // throw new ArgumentException("val1Start or val1Len is -1.");
                }
                var val1 = str.Substring(val1Start, val1Len);
                if (!int.TryParse(val1, out int num1)) continue;
                // Console.WriteLine($"val1: {val1}");

                var val2Start = str.IndexOf(',', indicies[i]) + 1;
                var val2Len = str.IndexOf(')', indicies[i]) - val2Start;
                if (val2Start < 0 || val2Len <= 0)
                {
                    continue;
                    // throw new ArgumentException("val2Start or val2Len is -1.");
                }
                var val2 = str.Substring(val2Start, val2Len);
                if (!int.TryParse(val2, out int num2)) continue;
                // Console.WriteLine($"val2: {val2}");

                //Console.WriteLine($"indicies[i]: {indicies[i]}");
                var subStr = str.Substring(0, indicies[i]);
                //Console.WriteLine($"subStr: {subStr}");
                if (doMul(subStr))
                {

                    muls.Add(new Tuple<int, int>(int.Parse(val1), int.Parse(val2)));
                }
                    
            }
            // Console.WriteLine($"index: {indicies[i]}");
        }
        return muls;
    } 


    private static bool doMul(string str)
    {
        // start as do the mul
        var doMul = true;

        // check the do's and don't up to the start index of the mul. i.e str = substring of full input.
        // flip the sign for every don't / do found
        // for (int i = 0; i < str.Length; i++)
        // {
        //     var foundDo = str.IndexOf("do()", i);
        //     var foundDont = str.IndexOf("don't()", i);
        //     if (foundDont != -1)
        //     {
        //         doMul = false;
        //         i = foundDont + 6;
        //         //Console.WriteLine($"don't() found at: {foundDont}");
        //     }
        //     if (foundDo != -1)
        //     {
        //         doMul = true;
        //         i = foundDo + 4;
        //         //Console.WriteLine($"do() found at: {foundDo}");
        //     }
            
        // }

        // reverse the check and only check the last one
        for (int i = str.Length - 1; i >= 0; i--)
        {
            var foundDo = str.IndexOf("do()", i);
            var foundDont = str.IndexOf("don't()", i);
            if (foundDont != -1)
            {
                return false;
            }
            if (foundDo != -1)
            {
                return true;
            }
        }

        // Console.WriteLine($"doMul: {doMul}");
        return doMul;
    }

    private static int ComputeMuls(List<Tuple<int, int>> muls)
    {
        if (muls.Count == 0)
        {
            throw new ArgumentException("Muls list is empty.");
        }

        int total = 0;
        for (int i = 0; i < muls.Count; i++)
        {
            Console.WriteLine($"mul: {muls[i].Item1} * {muls[i].Item2}");
            total += muls[i].Item1 * muls[i].Item2;
        }
        return total;
    }
}
