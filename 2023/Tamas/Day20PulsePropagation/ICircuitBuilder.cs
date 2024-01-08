namespace Day20PulsePropagation;

internal interface ICircuitBuilder
{
    Circuit Build(
        Bus bus,
        string? namePrefix = null);
}
