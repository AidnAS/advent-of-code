namespace Day08HauntedWasteland;

internal sealed class Map
{
    private readonly string instructions;
    private readonly IReadOnlyList<Node> nodes;
    private readonly Node start;
    private readonly IReadOnlyList<Node> ghostStartNodes;

    private Map(
        string instructions,
        IReadOnlyList<Node> nodes,
        Node start,
        IReadOnlyList<Node> ghostStartNodes)
    {
        this.instructions = instructions;
        this.nodes = nodes;
        this.start = start;
        this.ghostStartNodes = ghostStartNodes;
    }

    public int CountStepsToDestination()
    {
        int stepCount = 0;
        var node = start;
        int instructionIndex = 0;
        int maxStepCount = instructions.Length * nodes.Count;
        while (!node.IsDestination)
        {
            if (stepCount == maxStepCount)
            {
                throw NeverReached();
            }

            node = node.GetNext(instructions[instructionIndex]);
            instructionIndex = (instructionIndex + 1) % instructions.Length;
            stepCount++;
        }
        return stepCount;
    }

    public long CountStepsToGhostDestinations()
    {
        var ghosts = ghostStartNodes
            .Select(ghostStart => new Ghost() { SearchNode = ghostStart })
            .ToList();

        // Find repeating cycles for every ghost.
        long stepCount = 0;
        int instructionIndex = instructions.Length - 1;
        int maxStepCount = instructions.Length * nodes.Count;
        int searchingGhostCount = ghosts.Count;
        while (true)
        {
            if (stepCount == maxStepCount)
            {
                throw NeverReached();
            }
            stepCount++;
            instructionIndex = (instructionIndex + 1) % instructions.Length;
            var instruction = instructions[instructionIndex];

            bool isAllAtDestination = true;
            foreach (var ghost in ghosts)
            {
                if (ghost.IsCycleFound)
                {
                    isAllAtDestination &= ghost.IsAtDestination(stepCount);
                    continue;
                }

                var node = ghost.SearchNode!;
                node = node.GetNext(instruction);
                ghost.SearchNode = node;

                if (node.IsGhostDestination)
                {
                    ghost.RegisterDestination(
                        instructionIndex, node, stepCount);
                    if (ghost.IsCycleFound)
                    {
                        searchingGhostCount--;
                    }
                    continue;
                }

                isAllAtDestination = false;
            }

            if (isAllAtDestination)
            {
                return stepCount;
            }
            if (searchingGhostCount == 0)
            {
                break;
            }
        }

        if (ghosts.All(
            ghost => ghost.DestinationCountInCycle == 1))
        {
            // TODO: Implement efficient solution for special case based on the Chinese remainder theorem.
        }

        // Order ghosts for more efficient search.
        ghosts = ghosts
            .OrderByDescending(ghost => ghost.AverageCycleDestinationDistance)
            .ToList();

        // Jump from destination to destination with the leader ghost and check if the others
        // are also at destination nodes, using the cycles found earlier.
        var leaderGhost = ghosts[0];
        var stepCountEnumerator = leaderGhost.GetCyclicDestinationStepCountEnumerator(stepCount);
        while (true)
        {
            stepCountEnumerator.MoveNext();
            stepCount = stepCountEnumerator.Current;
            bool isAllAtDestination = true;
            for (int i = 1; i < ghosts.Count; i++)
            {
                var ghost = ghosts[i];
                if (!ghost.IsAtDestination(stepCount))
                {
                    isAllAtDestination = false;
                    break;
                }
            }
            
            if (isAllAtDestination)
            {
                return stepCount;
            }

            // TODO: Check if we reached the least common multiple and break.
        }

        throw NeverReached();
    }

    public static Map LoadFrom(
        string path)
    {
        var allLines = File.ReadAllLines(path);
        var instructions = allLines[0];
        var nodeMap = new Dictionary<string, Node>(allLines.Length - 2);
        var nodeList = new List<Node>(allLines.Length - 2);
        for (int i = 2; i < allLines.Length; i++)
        {
            string line = allLines[i];
            if (line.Length == 0)
            {
                continue;
            }

            var current = GetOrAddNode(nodeMap, line[..3]);
            current.IsGhostDestination = current.Label[2] == 'Z';
            current.Left = GetOrAddNode(nodeMap, line[7..10]);
            current.Right = GetOrAddNode(nodeMap, line[12..15]);
            nodeList.Add(current);
        }

        if (!nodeMap.TryGetValue("AAA", out var start))
        {
            start = nodeList[0];
        }

        if (!nodeMap.TryGetValue("ZZZ", out var destination))
        {
            destination = nodeList[^1];
        }
        destination.IsDestination = true;
        
        var ghostStartNodes = nodeMap.Values
            .Where(n => n.Label[2] == 'A')
            .ToList();

        return new Map(
            instructions,
            nodeList,
            start,
            ghostStartNodes);
    }

    private static Node GetOrAddNode(
        Dictionary<string, Node> nodeMap,
        string label)
    {
        if (!nodeMap.TryGetValue(label, out var node))
        {
            node = new Node(label);
            nodeMap.Add(label, node);
        }
        return node;
    }

    private static InvalidOperationException NeverReached()
    {
        return new InvalidOperationException("Never reached.");
    }
}
