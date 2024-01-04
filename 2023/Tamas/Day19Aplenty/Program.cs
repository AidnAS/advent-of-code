using Day19Aplenty;

var allLines = File.ReadAllLines("input.txt").AsSpan();
int emptyLineIndex = -1;
for (int i = 0; i < allLines.Length; i++)
{
    if (allLines[i].Length == 0)
    {
        emptyLineIndex = i;
        break;
    }
}
var workflows = WorkflowCollection.Parse(allLines[..emptyLineIndex]);
var parts = new List<Part>(allLines.Length - emptyLineIndex);
for (int i = emptyLineIndex + 1; i < allLines.Length; i++)
{
    string line = allLines[i];
    if (line.Length == 0)
    {
        continue;
    }
    
    parts.Add(
        Part.Parse(line));
}

int sum = parts
    .Where(workflows.Evaluate)
    .Sum(part => part.AttributeSum);
Console.WriteLine(
    $"The sum of all attributes of all accepted parts is {sum};");

ulong acceptedCount = workflows.CountAcceptedCombinations();
Console.WriteLine(
    $"The number of accepted combinations is {acceptedCount}.");
