using csharp.utils;

namespace csharp.Puzzles;

public static class Dec04
{
    public static char[,]? Matrix;

    public static int solvePart1(string? date, bool useTestData = false)
    {
        // log current puzzle
        Console.WriteLine("Day 04, Part 1: XMAS\n");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        // create a matrix from the data
        Matrix = CreateMatrix(dfr.Lines);

        // print the matrix
        // PrintMatrix(Matrix);

        var total = ScanMatrix(Matrix);
        
        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {total}\n");

		return total;
    }

    public static int solvePart2(string date, bool useTestData = false)
    {
        // log current puzzle
        Console.WriteLine("Day 04, Part 2: X-MAS\n");

        // create a data file reader and read the file.
        var dfr = new DataFileReader(date: date, useTestData: useTestData, part: 1);
        dfr.ReadFile();

        // create a matrix from the data
        Matrix = CreateMatrix(dfr.Lines);

        // print the matrix
        // PrintMatrix(Matrix);

        var total = TurboScan(Matrix);

        // log the solution
        var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
        Console.WriteLine($"{outputString}: {total}\n");

		return total;
    }


    private static char[,] CreateMatrix(List<string> lines)
    {   
        var numRows = lines.Count;
        var numCols = lines[0].ToCharArray().Length;

        char[,] matrix = new char[numRows, numCols];

        for (int i = 0; i < numRows; i++)
        {
            char[] col = lines[i].ToCharArray();

            for (int j = 0; j < numCols; j++)
            {
                matrix[i, j] = col[j];
            }
        }
        return matrix;
    }


    private static int ScanMatrix(char[,] matrix)
    {
        var numRows = matrix.GetLength(0);
        var numCols = matrix.GetLength(1);

        var count = 0;

        // pseudo code: Find XMAS
        // go char by char through the matrix.
        // check if the char is an X.
        // if yes, look in every direction to see if you can make the full word.
        // be aware of the edges of the matrix.

        for (int i = 0; i < numRows; i++)
        {
            for (int j = 0; j < numCols; j++)
            {
                if (matrix[i,j] == 'X')
                {
                    // check up left
                    if (i - 3 >= 0 && j - 3 >= 0)
                    {
                        if (matrix[i-1, j-1] == 'M' && matrix[i-2, j-2] == 'A' && matrix[i-3, j-3] == 'S')
                        {
                            count++;
                        }
                    }
                    // check up
                    if (i - 3 >= 0)
                    {
                        if (matrix[i-1, j] == 'M' && matrix[i-2, j] == 'A' && matrix[i-3, j] == 'S')
                        {
                            count++;
                        }
                    }
                    // check up right
                    if (i - 3 >= 0 && j + 3 < numCols)
                    {
                        if (matrix[i-1, j+1] == 'M' && matrix[i-2, j+2] == 'A' && matrix[i-3, j+3] == 'S')
                        {
                            count++;
                        }
                    }
                    // check right
                    if (j + 3 < numCols)
                    {
                        if (matrix[i, j+1] == 'M' && matrix[i, j+2] == 'A' && matrix[i, j+3] == 'S')
                        {
                            count++;
                        }
                    }
                    // check down right
                    if (i + 3 < numRows && j + 3 < numCols)
                    {
                        if (matrix[i+1, j+1] == 'M' && matrix[i+2, j+2] == 'A' && matrix[i+3, j+3] == 'S')
                        {
                            count++;
                        }
                    }
                    // check down
                    if (i + 3 < numRows)
                    {
                        if (matrix[i+1, j] == 'M' && matrix[i+2, j] == 'A' && matrix[i+3, j] == 'S')
                        {
                            count++;
                        }
                    }
                    // check down left
                    if (i + 3 < numRows && j - 3 >= 0)
                    {
                        if (matrix[i+1, j-1] == 'M' && matrix[i+2, j-2] == 'A' && matrix[i+3, j-3] == 'S')
                        {
                            count++;
                        }
                    }
                    // check left
                    if (j - 3 >= 0)
                    {
                        if (matrix[i, j-1] == 'M' && matrix[i, j-2] == 'A' && matrix[i, j-3] == 'S')
                        {
                            count++;
                        }
                    }
                }
            }
        }

        return count;
    }

    private static int TurboScan(char[,] matrix)
    {
        var numRows = matrix.GetLength(0);
        var numCols = matrix.GetLength(1);

        var count = 0;

        // psuedo code: Find X-MAS
        // go char by char through the matrix. (scanning down)
        // check if you can make the shape and if it is there. 
        // i.e. the char at [i,j] must be M or S to start the upper left corner of the shape.
        // be aware of the edges of the matrix again.
        // only count if ALL conditions are met.
        

        for (int i = 0; i < numRows; i++)
        {
            for (int j = 0; j < numCols; j++)
            {
                if (matrix[i,j] == 'M' || matrix[i,j] == 'S')
                {
                    // just actually check the possible values
                    if (BFCheck(matrix, i, j))
                    {
                        count++;
                    }
                }

            }
        }
        return count;
    }

    private static bool BFCheck(char[,] m, int i, int j)
    {
        var numRows = m.GetLength(0);
        var numCols = m.GetLength(1);

        // check if the shape can be made
        if (i+2 >= numRows || j+2 >= numCols) return false;

        // 1a. the letter is M
        // M.M
        // .A.
        // S.S
        var mm = m[i,j] == 'M' && m[i+1, j+1] == 'A' && m[i+2, j+2] == 'S' && m[i+2, j] == 'M' && m[i, j+2] == 'S';
        
        // 1b. the letter is M
        // M.S
        // .A.
        // M.S
        var ms = m[i,j] == 'M' && m[i+1, j+1] == 'A' && m[i+2, j+2] == 'S' && m[i+2, j] == 'S' && m[i, j+2] == 'M';

        // 2a. the letter is S
        // S.S
        // .A.
        // M.M
        var ss = m[i,j] == 'S' && m[i+1, j+1] == 'A' && m[i+2, j+2] == 'M' && m[i+2, j] == 'S' && m[i, j+2] == 'M';

        // 2b. the letter is S
        // S.M
        // .A.
        // S.M
        var sm = m[i,j] == 'S' && m[i+1, j+1] == 'A' && m[i+2, j+2] == 'M' && m[i+2, j] == 'M' && m[i, j+2] == 'S';

        return mm || ms || ss || sm;

    }

    private static void PrintMatrix(char[,] matrix)
    {
        var numRows = matrix.GetLength(0);
        var numCols = matrix.GetLength(1);

        for (int i = 0; i < numRows; i++)
        {
            for (int j = 0; j < numCols; j++)
            {
                Console.Write($" [{matrix[i,j]}] ");
            }
            Console.WriteLine("");
        }
    }


}
