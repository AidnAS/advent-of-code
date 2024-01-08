namespace Day20PulsePropagation;

internal abstract class Module
{
    private readonly Bus bus;
    
    private readonly List<Pin> downstreamPins;
    private int nextEmptyInputPin;

    protected Module(
        Bus bus,
        string name)
    {
        this.bus = bus;
        Name = name;

        downstreamPins = new List<Pin>();
        nextEmptyInputPin = 0;
    }

    public string Name { get; }
    public IReadOnlyList<Pin> DownstreamPins => downstreamPins;
    public abstract ModuleType ModuleType { get; }

    public Action<bool>? OnSignalReceived { get; set; }

    public void ConnectOutputTo(
        Module downstreamModule)
    {
        var downstreamPin = new Pin(
            downstreamModule, downstreamModule.nextEmptyInputPin++);
        downstreamPins.Add(downstreamPin);
        
        OnOutputConnected(downstreamPin);
        downstreamModule.OnInputConnected(this);
    }

    protected virtual void OnInputConnected(
        Module upstreamModule)
    {
    }

    protected virtual void OnOutputConnected(
        Pin downstreamPin)
    {
    }

    internal void ProcessSignal(
        Pin pin,
        bool signal)
    {
        OnSignalReceived?.Invoke(signal);

        var outputSignals = ProcessSignalCore(pin, signal);
        foreach (var outputSignal in outputSignals)
        {
            foreach (var downstreamPin in downstreamPins)
            {
                bus.SendSignal(
                    this, downstreamPin, outputSignal);
            }
        }
    }

    protected abstract IEnumerable<bool> ProcessSignalCore(
        Pin pin,
        bool signal);
}
