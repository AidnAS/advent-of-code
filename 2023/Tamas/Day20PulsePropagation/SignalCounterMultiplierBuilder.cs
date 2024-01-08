namespace Day20PulsePropagation;

internal sealed class SignalCounterMultiplierBuilder : CircuitBuilder
{
    public static readonly SignalCounterMultiplierBuilder Default = new();

    protected override IReadOnlyList<string> OutputNames => Array.Empty<string>();

    protected override void BuildCore()
    {
        var input = CreateModule(ModuleType.Broadcaster, "in");
        var accumulator = BuildSubCircuit(BlueprintCatalog.Int64, "acc-");
        BuildHalf("high-", ModuleType.Conjunction, input, accumulator);
        BuildHalf("low-", ModuleType.Broadcaster, input, accumulator);
    }

    private void BuildHalf(
        string prefix,
        ModuleType entryType,
        Module input,
        Circuit accumulator)
    {
        var entry = CreateModule(entryType, "e", prefix);
        input.ConnectOutputTo(entry);
        var number = BuildCounter(prefix, entry);
        var controlConnectors = BuildControlSignal(prefix, entry);
        
        for (int i = 0; i < 64; i++)
        {
            string idNumber = $"{i:D2}";
            var accumulatorBit = accumulator[$"i64b{idNumber}"];
            var numberOut = number[$"i64o{idNumber}"];
            numberOut.ConnectOutputTo(accumulatorBit);
            var controlConnector = controlConnectors[i];
            controlConnector.ConnectOutputTo(numberOut);
        }
    }

    private Circuit BuildCounter(
        string prefix,
        Module input)
    {
        var (numberDelayIn, numberDelayOut) = BuildDelay(4, prefix);
        input.ConnectOutputTo(numberDelayIn);
        var number = BuildSubCircuit(BlueprintCatalog.Int64, prefix);
        number.ConnectAllInputsTo(numberDelayOut);
        return number;
    }

    private IReadOnlyList<Module> BuildControlSignal(
        string prefix,
        Module input)
    {
        var reset = BuildSubCircuit(BlueprintCatalog.HighReset, prefix);
        reset.ConnectAllInputsTo(input);

        var resetConnectors = new Module[64];
        Module? previous = null;
        for (int i = 0; i < resetConnectors.Length; i++)
        {
            var module = CreateModule(ModuleType.Broadcaster, $"rc{i:D2}", prefix);
            previous?.ConnectOutputTo(module);
            resetConnectors[i] = module;
            previous = module;
        }
        reset.ConnectAllOutputsTo(resetConnectors[0]);

        return resetConnectors;
    }
}
