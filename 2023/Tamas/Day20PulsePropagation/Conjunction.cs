namespace Day20PulsePropagation;

internal sealed class Conjunction : Module
{
    private readonly List<bool> inputStates;
    private int highCount;

    public Conjunction(
        Bus bus,
        string name)
        : base(bus, name)
    {
        inputStates = new List<bool>();
    }

    public override ModuleType ModuleType => ModuleType.Conjunction;

    protected override void OnInputConnected(Module upstreamModule)
    {
        inputStates.Add(false);
    }

    protected override IEnumerable<bool> ProcessSignalCore(
        Pin pin,
        bool signal)
    {
        bool oldState = inputStates[pin.Number];
        if (oldState != signal)
        {
            inputStates[pin.Number] = signal;
            highCount += signal ? 1 : -1;
        }
        yield return !(highCount == inputStates.Count);
    }
}
