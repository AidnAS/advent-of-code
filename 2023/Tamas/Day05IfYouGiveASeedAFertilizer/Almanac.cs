namespace Day05IfYouGiveASeedAFertilizer;

internal sealed class Almanac
{
    private readonly IReadOnlyList<uint> seeds;
    private readonly IReadOnlyList<CategoryMapping> categoryMappings;

    private Almanac(
        IReadOnlyList<uint> seeds,
        IReadOnlyList<CategoryMapping> categoryMappings)
    {
        this.seeds = seeds;
        this.categoryMappings = categoryMappings;
    }

    public uint FindClosestLocation()
    {
        return seeds.Min(CalculateLocation);
    }

    public uint FindProperClosestLocation()
    {
        return CalculateLocationIntervals()
            .Min(interval => interval.Start);
    }

    private IEnumerable<Interval> CalculateLocationIntervals()
    {
        var sources = new List<Interval>();
        for (int i = 0; i < seeds.Count; i += 2)
        {
            sources.Add(new Interval(seeds[0], seeds[1]));
        }

        foreach (var mapping in categoryMappings)
        {
            sources = mapping.GetDestinationIntervals(sources)
                .ToList();
        }
        return sources;
    }

    private uint CalculateLocation(
        uint seed)
    {
        uint source = seed;
        foreach (var mapping in categoryMappings)
        {
            source = mapping.GetDestination(source);
        }
        return source;
    }

    public static Almanac ParseFrom(
        string fileName)
    {
        var allLines = File.ReadAllLines(fileName);
        var parts = allLines[0].Split(':');
        var seeds = parts[1]
            .Split(' ', StringSplitOptions.RemoveEmptyEntries)
            .Select(uint.Parse)
            .ToList();

        var mappings = new List<CategoryMapping>();
        string? heading = null;
        List<Rule> rules = null!;
        for (int i = 1; i < allLines.Length; i++)
        {
            string line = allLines[i];
            if (line.Length == 0)
            {
                if (i == allLines.Length - 1)
                {
                    break;
                }

                if (heading != null)
                {
                    rules.Sort(RuleSourceStartComparer.Default);
                    mappings.Add(
                        new CategoryMapping(
                            heading, rules));
                }
                
                i++;
                line = allLines[i];
                parts = line.Split(' ', 2);
                heading = parts[0];
                rules = new List<Rule>();
                continue;
            }

            parts = line.Split(' ');
            rules.Add(new Rule(
                uint.Parse(parts[0]),
                uint.Parse(parts[1]),
                uint.Parse(parts[2])));
        }

        if (heading != null)
        {
            rules.Sort(RuleSourceStartComparer.Default);
            mappings.Add(
                new CategoryMapping(
                    heading, rules));
        }

        return new Almanac(seeds, mappings);
    }
}
