using Day13PointOfIncidence;

var allPatterns = LoadPatterns("input.txt");
var summaryValue = allPatterns.Sum(pattern => pattern.GetSummaryValue(fixingSmudge: false));
Console.WriteLine($"The value of the summarized notes is {summaryValue}.");
summaryValue = allPatterns.Sum(pattern => pattern.GetSummaryValue(fixingSmudge: true));
Console.WriteLine($"The value of the summarized notes after fixing all smudges is {summaryValue}.");

static IReadOnlyList<Pattern> LoadPatterns(
    string path)
{
    var allLines = File.ReadAllLines(path);
    var patternList = new List<Pattern>();
    int? patternStartIndex = null;
    for (int i = 0; i < allLines.Length; i++)
    {
        var line = allLines[i];
        if (line.Length == 0)
        {
            if (patternStartIndex.HasValue)
            {
                patternList.Add(
                    new(allLines[patternStartIndex.Value..i]));
                patternStartIndex = null;
                continue;
            }
        }
        if (!patternStartIndex.HasValue)
        {
            patternStartIndex = i;
        }
    }
    if (patternStartIndex.HasValue)
    {
        patternList.Add(
            new(allLines[patternStartIndex.Value..]));
    }
    return patternList;
}