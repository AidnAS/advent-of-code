namespace Day20PulsePropagation;

internal sealed class Button : Module
{
    private readonly bool generatesSignal;
    private readonly Pin virtualPin;

    public Button(
        Bus bus,
        string name,
        bool generatesSignal = false)
    : base(bus, name)
    {
        this.generatesSignal = generatesSignal;
        virtualPin = new Pin(this, -1);
    }

    public override ModuleType ModuleType => ModuleType.Button;

    public void Push()
    {
        ProcessSignal(virtualPin, generatesSignal);
    }

    protected override IEnumerable<bool> ProcessSignalCore(
        Pin pin, 
        bool signal)
    {
        yield return signal;
    }
}
