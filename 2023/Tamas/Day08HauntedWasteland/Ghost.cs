namespace Day08HauntedWasteland;

internal sealed class Ghost
{
    private readonly Dictionary<InstructionIndexAndNode, long> cycleSearchMap = new();

    private readonly HashSet<long> prefixDestinationStepSet = new();
    private readonly HashSet<long> cycleDestinationStepSet = new();
    
    public bool IsCycleFound { get; private set; }
    public Node? SearchNode { get; set; }
    public long CycleStartStepCount { get; private set; } = -1;
    public long CycleLength { get; private set; } = -1;

    public int DestinationCountInCycle => cycleDestinationStepSet.Count;
    public long AverageCycleDestinationDistance => CycleLength / DestinationCountInCycle;

    public bool IsAtDestination(
        long stepCount)
    {
        if (stepCount < CycleStartStepCount)
        {
            return prefixDestinationStepSet.Contains(
                stepCount);
        }
        else
        {
            return cycleDestinationStepSet.Contains(
                (stepCount - CycleStartStepCount) % CycleLength);
        }
    }

    public IEnumerator<long> GetCyclicDestinationStepCountEnumerator(
        long startStepCount)
    {
        var cycleStepCountsSorted = cycleDestinationStepSet
            .OrderBy(s => s)
            .ToList();
        long startCycleStepCount = startStepCount % CycleLength;
        int firstDestinationCycleStepCountIndex = 0;
        for (int i = 0; i < cycleStepCountsSorted.Count; i++)
        {
            long candidateStepCount = cycleStepCountsSorted[i];
            if (candidateStepCount > startCycleStepCount)
            {
                firstDestinationCycleStepCountIndex = i;
                break;
            }
        }
        long firstDestinationCycleStepCount = cycleStepCountsSorted[firstDestinationCycleStepCountIndex];
        long stepCount = startStepCount;
        stepCount += firstDestinationCycleStepCount == 0
            ? CycleLength - startCycleStepCount
            : firstDestinationCycleStepCount - startCycleStepCount;
        yield return stepCount;

        var stepCountIncrements = new long[cycleStepCountsSorted.Count];
        stepCountIncrements[0] = CycleLength - cycleStepCountsSorted[^1];
        for (int i = 1; i < stepCountIncrements.Length; i++)
        {
            stepCountIncrements[i] = cycleStepCountsSorted[i] - cycleStepCountsSorted[i - 1];
        }

        int destinationCount = DestinationCountInCycle;
        int index = firstDestinationCycleStepCountIndex;
        while (true)
        {
            index = (index + 1) % destinationCount;
            stepCount += stepCountIncrements[index];
            yield return stepCount;
        }
    }

    public void RegisterDestination(
        int instructionIndex,
        Node destination,
        long stepCount)
    {
        var key = new InstructionIndexAndNode(instructionIndex, destination);
        IsCycleFound = cycleSearchMap.TryGetValue(key, out long previousStepCount);
        if (IsCycleFound)
        {
            CycleStartStepCount = previousStepCount;
            CycleLength = stepCount - previousStepCount;
            foreach (var destinationStep in cycleSearchMap.Values)
            {
                if (destinationStep < CycleStartStepCount)
                {
                    prefixDestinationStepSet.Add(destinationStep);
                }
                else
                {
                    cycleDestinationStepSet.Add(destinationStep - CycleStartStepCount);
                }
            }
        }
        else
        {
            cycleSearchMap.Add(key, stepCount);
        }
    }
}
