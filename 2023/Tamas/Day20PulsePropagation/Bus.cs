namespace Day20PulsePropagation;

internal sealed class Bus
{
    private readonly Queue<SendRequest> signalQueue;

    public Bus()
    {
        signalQueue = new Queue<SendRequest>();
    }

    public bool HasWaitingSignals => signalQueue.Count > 0;

    public void SendSignal(
        Module from,
        Pin toPin,
        bool signal)
    {
        var request = new SendRequest(from, toPin, signal);
        signalQueue.Enqueue(
            request);
    }

    public void Run()
    {
        while (PropagateNext()) { }
    }

    public bool PropagateNext()
    {
        if (!HasWaitingSignals)
        {
            return false;
        }

        var request = signalQueue.Dequeue();
        request.ToPin.Module.ProcessSignal(
            request.ToPin, request.Signal);
        return true;
    }

    private sealed record SendRequest(
        Module FromModule,
        Pin ToPin,
        bool Signal)
    {
        public override string ToString()
        {
            return $"{FromModule.Name} -{(Signal ? "high" : "low")}-> {ToPin.Module.Name}";
        }
    }
}
