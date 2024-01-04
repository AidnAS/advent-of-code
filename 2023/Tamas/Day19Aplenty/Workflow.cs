namespace Day19Aplenty;

internal sealed record Workflow(
    string Name,
    IReadOnlyList<Rule> Rules);
