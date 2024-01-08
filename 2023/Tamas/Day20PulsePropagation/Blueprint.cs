namespace Day20PulsePropagation;

internal sealed class Blueprint : ICircuitBuilder
{
    private static readonly char[] LineSeparators = new char[] { '\r', '\n' };

    private readonly string[] moduleDescriptions;
    private readonly IReadOnlyList<string> inputNames;
    private readonly IReadOnlyList<string> outputNames;

    public Blueprint(
        string moduleDescriptions,
        IReadOnlyList<string> inputNames,
        IReadOnlyList<string> outputNames)
    {
        this.moduleDescriptions = moduleDescriptions.Split(
            LineSeparators, StringSplitOptions.RemoveEmptyEntries);
        this.inputNames = inputNames;
        this.outputNames = outputNames;
    }

    public Blueprint(
        string moduleDescriptions,
        string inputName = "in",
        string outputName = "out")
        : this(
              moduleDescriptions, 
              new string[] { inputName }, 
              new string[] { outputName })
    {
    }

    public Circuit Build(
        Bus bus,
        string? namePrefix = null)
    {
        var moduleList = new List<Module>(moduleDescriptions.Length);
        var moduleMap = new Dictionary<string, Module>(moduleDescriptions.Length);
        foreach (var description in moduleDescriptions)
        {
            if (description.Length == 0)
            {
                break;
            }

            int nameStartIndex = description[0] is '%' or '&' ? 1 : 0;
            int nameEndIndex = description.IndexOf(' ');
            if (nameEndIndex == -1) { nameEndIndex = description.Length; }
            string name = description[nameStartIndex..nameEndIndex];
            string fullName = namePrefix + name;
            Module module = description[0] switch
            {
                '%' => new FlipFlop(bus, fullName),
                '&' => new Conjunction(bus, fullName),
                _ => new Broadcaster(bus, fullName)
            };
            moduleList.Add(module);
            moduleMap.Add(name, module);
        }

        for (int i = 0; i < moduleList.Count; i++)
        {
            var module = moduleList[i];
            string description = moduleDescriptions[i];
            int arrowIndex = description.IndexOf(' ');
            if (arrowIndex != -1)
            {
                var downstreamParts = description[(arrowIndex + 4)..].Split(", ");
                foreach (var part in downstreamParts)
                {
                    if (!moduleMap.TryGetValue(part, out var downstreamModule))
                    {
                        // For the non-existent "output" module in example2.
                        downstreamModule = new Broadcaster(bus, part);
                        moduleMap.Add(part, downstreamModule);
                    }

                    module.ConnectOutputTo(downstreamModule);
                }
            }
        }

        return new Circuit(
            moduleMap,
            inputNames.Select(name => moduleMap[name]).ToList(),
            outputNames.Select(name => moduleMap[name]).ToList());
    }
}
