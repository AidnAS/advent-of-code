namespace Day20PulsePropagation;

internal sealed class Broadcaster : Module
{
    public Broadcaster(
        Bus bus,
        string name)
        : base(bus, name)
    {
    }

    public override ModuleType ModuleType => ModuleType.Broadcaster;

    protected override IEnumerable<bool> ProcessSignalCore(
        Pin pin, 
        bool signal)
    {
        yield return signal;
    }
}
