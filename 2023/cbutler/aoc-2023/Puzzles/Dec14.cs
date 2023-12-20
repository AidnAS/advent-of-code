using aoc_2023.Entities;
using aoc_2023.Services;

namespace aoc_2023.Puzzles;

public class Rock
{
    private string[] AllowedValues = new string[] { "O", "#", ".", "-" };

    public string Type { get; set; } = "";
    public Pos Position = new Pos();

    public Rock(string type, Pos pos)
    {
        SetType(type);
        Position.X = pos.X;
        Position.Y = pos.Y;
    }

    private void SetType(string value)
    {
        if (!AllowedValues.Any(o => o == value) || string.IsNullOrWhiteSpace(value))
            throw new ArgumentException($"Not a valid tile value: {value}");

        Type = value;
    }
}

public class Platform
{
    public List<string> Rows = new List<string>();
    public List<string> Cols = new List<string>();

    public char[,] Matrix;

    public Platform(List<string> lines)
    {
        InitializePlatform(lines);
        Matrix = InitializeGrid(lines);
    }

    #region Part 1

    private void InitializePlatform(List<string> lines)
    {
        for (int i = 0; i < lines.Count; i++)
        {
            Rows.Add(lines[i]);
        }

        var numCols = lines[0].ToCharArray().Count();

        for (int i = 0; i < numCols; i++)
        {
            var col = "";
            for (int j = 0; j < lines.Count; j++)
            {
                col += lines[j].ToCharArray().Select(c => c.ToString()).ToArray()[i];
            }
            Cols.Add(col);
        }
    }

    public void TiltLeverNorth()
    {
        // the column
        for (int i = 0; i < Cols.Count(); i++)
        {
            // each character in the column
            for (int j = 0; j < Cols[i].Count(); j++)
            {
                var currCol = Cols[i];

                // boulders don't move
                if (currCol[j].ToString() == "#") continue;

                // swap boulder with empty spot
                if (currCol[j].ToString() == ".")
                {
                    var (nextRockIdx, nextRockVal) = FindNextRock(currCol, j + 1);

                    // no more rocks to move
                    if (nextRockIdx == -1) continue;

                    // skip past next bounder
                    if (nextRockVal == "#")
                    {
                        // move past next boulder;
                        j = nextRockIdx;
                        continue;
                    }
                    Cols[i] = SwapItems(Cols[i], indexA: j, valA: currCol[j].ToString(), indexB: nextRockIdx, valB: currCol[nextRockIdx].ToString());
                }

            }
        }
    }

    public int ComputeLoad()
    {
        int load = 0;

        for (int i = 0; i < Cols.Count(); i++)
        {
            for (int j = 0; j < Cols[i].Count(); j++)
            {
                if (Cols[i][j].ToString() == "O")
                    load += Cols[i].Count() - j;
            }
        }
        return load;
    }

    private (int idx, string type) FindNextRock(string items, int index)
    {
        for (int i = index; i < items.ToCharArray().Count(); i++)
        {
            if (items[i].ToString() == "#") return (i, "#");
            if (items[i].ToString() == "O") return (i, "O");
        }
        return (-1, "-");
    }

    // use the same fucntion for rows and cols. items = the row or col
    private string SwapItems(string items, int indexA, string valA, int indexB, string valB)
    {
        var chars = items.ToCharArray();

        var tmp = chars[indexA];
        chars[indexA] = chars[indexB];
        chars[indexB] = tmp;

        return new string(chars);
    }

    #endregion


    #region Part 2

    private char[,] InitializeGrid(List<string> lines)
    {
        var numRows = lines.Count;
        var numCols = lines[0].ToCharArray().Count();

        char[,] matrix = new char[numRows, numCols];

        for (int i = 0; i < lines.Count; i++)
        {
            char[] col = lines[i].ToCharArray();

            for (int j = 0; j < col.Length; j++)
            {
                matrix[i, j] = col[j];
            }
        }
        return matrix;
    }

    public void TiltGridNorth()
    {
        // traverse the columns
        for (int i = 0; i < Matrix.GetLength(1); i++)
        {
            // traverse the rows
            for (int j = 0; j < Matrix.GetLength(0); j++)
            {
                // Console.Write($" [{Matrix[j, i]}] ");

                // boulders don't move
                if (Matrix[j, i].ToString() == "#") continue;

                // swap boulder with empty spot
                if (Matrix[j, i].ToString() == ".")
                {
                    var currCol = Enumerable.Range(0, Matrix.GetLength(0))
                                            .Select(x => Matrix[x, i])
                                            .ToArray();

                    var (nextRockIdx, nextRockVal) = FindNextGridRock(currCol, j + 1);

                    // no more rocks to move
                    if (nextRockIdx == -1) continue;

                    // skip past next bounder
                    if (nextRockVal == "#" && nextRockIdx <= Matrix.GetLength(0))
                    {
                        // move past next boulder;
                        j = nextRockIdx;
                        continue;
                    }
                    SwapVerticalMatrixItems(iA: i, jA: j, iB: i, jB: nextRockIdx);
                }

                // Console.Write($" [{Matrix[j, i]}] ");

            }
            //Console.WriteLine("");
            //Console.WriteLine("");
        }
    }

    public void TiltGridWest()
    {
        // traverse the rows
        for (int i = 0; i < Matrix.GetLength(0); i++)
        {
            // traverse the columns
            for (int j = 0; j < Matrix.GetLength(1); j++)
            {
                // boulders don't move
                if (Matrix[i, j].ToString() == "#") continue;

                if (Matrix[i, j].ToString() == ".")
                {
                    var currRow = Enumerable.Range(0, Matrix.GetLength(1))
                                            .Select(x => Matrix[i, x])
                                            .ToArray();

                    var (nextRockIdx, nextRockVal) = FindNextGridRock(currRow, j + 1);

                    // no more rocks to move
                    if (nextRockIdx == -1) continue;

                    // skip past next bounder
                    if (nextRockVal == "#" && nextRockIdx <= Matrix.GetLength(1))
                    {
                        // move past next boulder;
                        j = nextRockIdx;
                        continue;
                    }

                    SwapHorizontalMatrixItems(iA: i, jA: j, iB: i, jB: nextRockIdx);

                }

            }
        }
    }

    public void TiltGridSouth()
    {
        // traverse the columns
        for (int i = 0; i < Matrix.GetLength(1); i++)
        {
            // traverse the rows in reverse
            for (int j = Matrix.GetLength(0) - 1; j >= 0; j--)
            {
                // Console.Write($" [{Matrix[j, i]}] ");

                // boulders don't move
                if (Matrix[j, i].ToString() == "#") continue;

                // swap boulder with empty spot
                if (Matrix[j, i].ToString() == ".")
                {
                    var currCol = Enumerable.Range(0, Matrix.GetLength(0))
                                            .Select(x => Matrix[x, i])
                                            .ToArray();

                    var (nextRockIdx, nextRockVal) = FindNextGridRockReverse(currCol, j);

                    // no more rocks to move
                    if (nextRockIdx == -1) continue;

                    // skip past next bounder
                    if (nextRockVal == "#" && nextRockIdx >= 0)
                    {
                        // move past next boulder;
                        j = nextRockIdx;
                        continue;
                    }
                    SwapVerticalMatrixItems(iA: i, jA: j, iB: i, jB: nextRockIdx);
                }

                // Console.Write($" [{Matrix[j, i]}] ");

            }
            //Console.WriteLine("");
            //Console.WriteLine("");
        }
    }

    public void TiltGridEast()
    {
        // traverse the rows
        for (int i = 0; i < Matrix.GetLength(0); i++)
        {
            // traverse the columns in reverse
            for (int j = Matrix.GetLength(1) - 1; j >= 0; j--)
            {
                // boulders don't move
                if (Matrix[i, j].ToString() == "#") continue;

                if (Matrix[i, j].ToString() == ".")
                {
                    var currRow = Enumerable.Range(0, Matrix.GetLength(1))
                                            .Select(x => Matrix[i, x])
                                            .ToArray();

                    var (nextRockIdx, nextRockVal) = FindNextGridRockReverse(currRow, j);

                    // no more rocks to move
                    if (nextRockIdx == -1) continue;

                    // skip past next bounder
                    if (nextRockVal == "#" && nextRockIdx >= 0)
                    {
                        // move past next boulder;
                        j = nextRockIdx;
                        continue;
                    }

                    SwapHorizontalMatrixItems(iA: i, jA: j, iB: i, jB: nextRockIdx);

                }

            }
        }
    }

    private void SwapVerticalMatrixItems(int iA, int jA, int iB, int jB)
    {
        var tmp = Matrix[jA, iA];
        Matrix[jA, iA] = Matrix[jB, iB];
        Matrix[jB, iB] = tmp;
    }

    private void SwapHorizontalMatrixItems(int iA, int jA, int iB, int jB)
    {
        var tmp = Matrix[iA, jA];
        Matrix[iA, jA] = Matrix[iB, jB];
        Matrix[iB, jB] = tmp;
    }

    public int SpinCycle(int cycles = 1)
    {
        // track the loads
        List<int> loads = new List<int>();

        for (int i = 0; i < cycles; i++)
        {
            TiltGridNorth();
            // PrintGrid();
            TiltGridWest();
            // PrintGrid();
            TiltGridSouth();
            // PrintGrid();
            TiltGridEast();
            // PrintGrid();

            // track loads
            loads.Add(ComputeGridLoad());

            // Console.WriteLine($"i: {i} - load: {loads[loads.Count()-1]}");

            // addign a shit ton of comments so I can refer back next year for a similar problem.

            // sequenceStartIndex - The start of the repeating sequence
            // length - the length of the repeating sequence
            (int sequenceStartIndex, int length) = FindSequence(loads);
            // we need to check if a repeating sequence was actually found.
            // FindSequence will return -1 if no sequence found.
            if (sequenceStartIndex >= 0)
            {
                // sequenceOffsetIndex - The calculated offset from the start of the repeating sequence.
                int sequenceOffsetIndex = (cycles - sequenceStartIndex) % length;

                // this is the index of the value that corresponds to when cycles is at	1000000000.
                var absoluteIndex = sequenceStartIndex + sequenceOffsetIndex;

                // the -1 is just to adjust for zero based indexing.
                // We could also subtract one from cycles to make it 999999999
                return loads[absoluteIndex - 1];
            }
        }
        // sign that we didn't find an answer
        return -1;
    }

    private (int sequenceStartIndex, int length) FindSequence(List<int> loads)
    {
        // This variable sets the minimum length of the sequence considered.
        // we want to avoid sequences smaller than 5.
        int minSequenceLength = 5;

        // this variable is the end of the list and is a variable for convenience. it doesn't change.
        // We start the search from the end of the list and count down to the beginning (or middle.)
        int lastItemIndex = loads.Count - 1;

        // we will compare value this "far apart" to see if there is a sequence
        // You need to start at least `minSequenceLength` away because the list cannot repeat (or we do not consider)
        // shorter repetitions
        int lastItemOffsetIndex = lastItemIndex - minSequenceLength;

        // this is simply an optimization to reduce the search space and make the search faster
        // we don't need to search the entire collection to find a repeating sequence
        int minSearchIndex = (loads.Count / 2) - 1;


        // start counting 5 from the end and stop short of the min
        for (int i = lastItemOffsetIndex; i >= minSearchIndex; i--)
        {
            // we use two indicies to compare values from the loads list.
            // the key here is that the length of the sequence starts at 5
            // because of the math for lastItemOffsetIndex (if minSequence was higher, or lower this would follow)
            int rightComparerIndex = lastItemIndex;

            // this is really the key. As i updates, the length of the sequence checked changes (gets longer) as well.
            // by doing this, we can check different length of sequences, starting from length of 5.
            int leftComparerIndex = i;
            // as the comparers check values, they slide down the list until the right comparer reaches i.
            while (rightComparerIndex > i && loads[leftComparerIndex--] == loads[rightComparerIndex--]) { }
            // if the right comparer is equal to i, we have found a repeating sequence as long as or longer than our minSequenceLength.
            if (rightComparerIndex == i)
            {
                // we return the starting index of the repeating sequence and the length of the sequence.
                // we could make a variable for this for clarity but we want this to be fast
                return (i, lastItemIndex - rightComparerIndex);
            }
        }

        // no sequence found
        return (-1, 0);
    }


    private (int idx, string type) FindNextGridRock(char[] items, int index)
    {
        for (int i = index; i < items.Count(); i++)
        {
            if (items[i].ToString() == "#") return (i, "#");
            if (items[i].ToString() == "O") return (i, "O");
        }
        return (-1, "-");
    }

    private (int idx, string type) FindNextGridRockReverse(char[] items, int index)
    {
        for (int i = index; i >= 0; i--)
        {
            if (items[i].ToString() == "#") return (i, "#");
            if (items[i].ToString() == "O") return (i, "O");
        }
        return (-1, "-");
    }

    public int ComputeGridLoad()
    {
        int load = 0;

        for (int i = 0; i < Matrix.GetLength(0); i++)
        {
            for (int j = 0; j < Matrix.GetLength(1); j++)
            {
                if (Matrix[i, j].ToString() == "O")
                    load += Matrix.GetLength(0) - i;
            }
        }
        return load;
    }

    public void PrintGrid()
    {
        for (int i = 0; i < Matrix.GetLength(0); i++)
        {
            for (int j = 0; j < Matrix.GetLength(1); j++)
            {
                Console.Write($" [{Matrix[i, j]}] ");
            }
            Console.WriteLine("");
            Console.WriteLine("");
        }
        Console.WriteLine("");
        Console.WriteLine("");
    }

    #endregion
}





public class Dec14
{
    public static void SolvePt1(string? date, bool useTestData = false)
    {
        var watch = System.Diagnostics.Stopwatch.StartNew();

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        Platform platform = new Platform(lines: dfr.Lines);
        platform.TiltLeverNorth();
        int total = platform.ComputeLoad();

        watch.Stop();
        var elapsedMs = watch.ElapsedMilliseconds;

        var outputString = useTestData ? "Part 1 [using test data]" : "Part 1 [using puzzle data]";
        Console.WriteLine($"{outputString}: {total} [elapsed time in ms: {elapsedMs}]");
    }

    public static void SolvePt2(string? date, bool useTestData = false)
    {
        var watch = System.Diagnostics.Stopwatch.StartNew();

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        Platform platform = new Platform(lines: dfr.Lines);
        // platform.PrintGrid();
        int total = platform.SpinCycle(cycles: 1000000000);

        watch.Stop();
        var elapsedMs = watch.ElapsedMilliseconds;

        var outputString = useTestData ? "Part 2 [using test data]" : "Part 2 [using puzzle data]";
        Console.WriteLine($"{outputString}: {total} [elapsed time in ms: {elapsedMs}]");
    }



}

