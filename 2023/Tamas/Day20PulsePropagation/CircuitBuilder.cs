namespace Day20PulsePropagation;

internal abstract class CircuitBuilder : ICircuitBuilder
{
    private static readonly string[] DefaultInputNames = new string[] { "in" };
    private static readonly string[] DefaultOutputNames = new string[] { "out" };

    private string outerPrefix = default!;
    private Dictionary<string, Module> moduleMap = default!;
    private Bus bus = default!;

    protected virtual IReadOnlyList<string> InputNames { get; } = DefaultInputNames;
    protected virtual IReadOnlyList<string> OutputNames { get; } = DefaultOutputNames;

    protected IReadOnlyDictionary<string, Module> ModuleMap => moduleMap;

    public Circuit Build(
        Bus bus, 
        string? namePrefix = null)
    {
        outerPrefix = namePrefix ?? string.Empty;
        moduleMap = new Dictionary<string, Module>();
        this.bus = bus;

        BuildCore();

        return new Circuit(
            moduleMap,
            InputNames.Select(name => moduleMap[name]).ToList(),
            OutputNames.Select(name => moduleMap[name]).ToList());
    }

    protected abstract void BuildCore();

    protected (Module In, Module Out) BuildDelay(
        int length,
        string? innerPrefix = null)
    {
        var input = CreateModule(ModuleType.Broadcaster, innerPrefix + "delay00");
        var current = input;
        for (int i = 1; i < length; i++)
        {
            var next = CreateModule(ModuleType.Broadcaster, innerPrefix + $"delay{i:D2}");
            current.ConnectOutputTo(next);
            current = next;
        }
        return (input, current);
    }

    protected Module CreateModule(
        ModuleType moduleType,
        string name, 
        string? innerPrefix = null)
    {
        string innerName = innerPrefix + name;
        string moduleName = outerPrefix + innerName;
        Module module = moduleType switch
        {
            ModuleType.Broadcaster => new Broadcaster(bus, moduleName),
            ModuleType.FlipFlop => new FlipFlop(bus, moduleName),
            ModuleType.Conjunction => new Conjunction(bus, moduleName),
            ModuleType.Button => new Button(bus, moduleName),
            _ => throw new ArgumentOutOfRangeException(nameof(moduleType))
        };
        moduleMap!.Add(innerName, module);
        return module;
    }

    protected Circuit BuildSubCircuit(
        ICircuitBuilder builder,
        string? innerPrefix = null)
    {
        var circuit = builder.Build(bus, outerPrefix + innerPrefix);
        foreach (var (name, module) in circuit.ModuleMap)
        {
            moduleMap!.Add(innerPrefix + name, module);
        }
        return circuit;
    }
}
