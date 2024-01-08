namespace Day20PulsePropagation;

internal sealed class FlipFlop : Module
{
    private bool state;

    public FlipFlop(
        Bus bus,
        string name)
        : base(bus, name)
    {
    }

    public override ModuleType ModuleType => ModuleType.FlipFlop;
    public bool State => state;

    protected override IEnumerable<bool> ProcessSignalCore(
        Pin pin, 
        bool signal)
    {
        if (signal == true)
        {
            yield break;
        }

        state = !state;
        yield return state;
    }
}
