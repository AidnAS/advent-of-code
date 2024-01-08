namespace Day20PulsePropagation;

internal sealed record Circuit(
    IReadOnlyDictionary<string, Module> ModuleMap,
    IReadOnlyList<Module> Inputs,
    IReadOnlyList<Module> Outputs)
{
    public Module this[string name] => ModuleMap[name];

    public void ConnectAllInputsTo(
        Module upstreamModule)
    {
        foreach (var module in Inputs)
        {
            upstreamModule.ConnectOutputTo(module);
        }
    }

    public void ConnectAllOutputsTo(
        Module downstreamModule)
    {
        foreach (var module in Outputs)
        {
            module.ConnectOutputTo(downstreamModule);
        }
    }
}
