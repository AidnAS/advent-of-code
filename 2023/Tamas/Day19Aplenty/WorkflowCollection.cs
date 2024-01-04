namespace Day19Aplenty;

internal sealed class WorkflowCollection
{
    private const string StartWorkflowName = "in";
    private const string AcceptString = "A";
    private const string RejectString = "R";
    private const int AcceptWorkflowNumber = -1;
    private const int RejectWorkflowNumber = -2;

    private readonly Workflow[] workflows;
    private readonly Workflow startWorkflow;

    private WorkflowCollection(
        Workflow[] workflows,
        Workflow startWorkflow)
    {
        this.workflows = workflows;
        this.startWorkflow = startWorkflow;
    }

    public bool Evaluate(
        Part part)
    {
        var workflow = startWorkflow;
        while (true)
        {
            for (int i = 0; i < workflow.Rules.Count; i++)
            {
                var rule = workflow.Rules[i];
                var ruleResult = rule.Evaluate(part);
                if (ruleResult.HasValue)
                {
                    if (ruleResult is AcceptWorkflowNumber or RejectWorkflowNumber) 
                    {
                        return ruleResult == AcceptWorkflowNumber; 
                    }

                    workflow = workflows[ruleResult.Value];
                    break;
                }
            }
        }
    }

    public ulong CountAcceptedCombinations()
    {
        ulong acceptedCount = 0;
        var queue = new Queue<Cursor>();
        var completeRanges = new AttributeRanges(
            new Range(1, 4000),
            new Range(1, 4000),
            new Range(1, 4000),
            new Range(1, 4000));

        queue.Enqueue(new Cursor(completeRanges, startWorkflow, 0));
        while (queue.Count > 0)
        {
            var (ranges, workflow, ruleIndex) = queue.Dequeue();
            var rule = workflow.Rules[ruleIndex];

            var (pass, fail) = rule.Evaluate(ranges);
            if (pass.HasValue)
            {
                int ruleResult = rule.Result;
                if (ruleResult == AcceptWorkflowNumber)
                {
                    acceptedCount += pass.Value.CombinationCount;
                }
                else if (ruleResult != RejectWorkflowNumber)
                {
                    var nextWorkflow = workflows[ruleResult];
                    queue.Enqueue(
                        new Cursor(pass.Value, nextWorkflow, 0));
                }
            }
            if (fail.HasValue)
            {
                queue.Enqueue(
                    new Cursor(fail.Value, workflow, ruleIndex + 1));
            }
        }
        return acceptedCount;
    }

    public static WorkflowCollection Parse(
        ReadOnlySpan<string> lines)
    {
        var workflowArray = new Workflow[lines.Length];
        var workflowMap = new Dictionary<string, int>(lines.Length);
        int nextWorkflowNumber = 0;
        foreach (var line in lines)
        {
            int leftCurlyIndex = line.IndexOf('{');
            string workflowName = line[0..leftCurlyIndex];
            int workflowNumber = GetWorkflowNumber(
                workflowName, workflowMap, ref nextWorkflowNumber);
            var ruleParts = line[(leftCurlyIndex + 1)..^1].Split(',');
            var rules = new Rule[ruleParts.Length];
            for (int i = 0; i < ruleParts.Length; i++)
            {
                rules[i] = ParseRule(
                    ruleParts[i], workflowMap, ref nextWorkflowNumber);
            }

            workflowArray[workflowNumber] = new Workflow(
                workflowName, rules);
        }

        int startWorkflowNumber = workflowMap[StartWorkflowName];
        var startWorkflow = workflowArray[startWorkflowNumber];
        return new WorkflowCollection(
            workflowArray, startWorkflow);

        static Rule ParseRule(
            string input,
            Dictionary<string, int> workflowMap,
            ref int nextWorkflowNumber)
        {
            int colonIndex = input.IndexOf(':');
            string resultWorkflowName = colonIndex != -1
                ? input[(colonIndex + 1)..]
                : input;
            int resultWorkflowNumber = GetWorkflowNumber(
                resultWorkflowName, workflowMap, ref nextWorkflowNumber);

            if (colonIndex == -1)
            {
                return Rule.CreateUnconditional(
                    resultWorkflowNumber);
            }

            var attribute = input[0] switch
            {
                'x' => Attribute.X,
                'm' => Attribute.M,
                'a' => Attribute.A,
                's' => Attribute.S,
                _ => throw new InvalidOperationException()
            };

            var relation = input[1] switch
            {
                '<' => Relation.LessThan,
                '>' => Relation.GreaterThan,
                _ => throw new InvalidOperationException()
            };

            int threshold = int.Parse(input[2..colonIndex]);
            
            return Rule.CreateConditional(
                resultWorkflowNumber, attribute, relation, threshold);
        }

        static int GetWorkflowNumber(
            string workflowName,
            Dictionary<string, int> workflowMap,
            ref int nextWorkflowNumber)
        {
            if (!TryGetAcceptOrRejectWorkflowNumber(workflowName, out int workflowNumber)
                && !workflowMap.TryGetValue(workflowName, out workflowNumber))
            {
                workflowNumber = nextWorkflowNumber++;
                workflowMap.Add(workflowName, workflowNumber);
            }
            return workflowNumber;
        }

        static bool TryGetAcceptOrRejectWorkflowNumber(
            string input,
            out int workflowNumber)
        {
            workflowNumber = input switch
            {
                AcceptString => AcceptWorkflowNumber,
                RejectString => RejectWorkflowNumber,
                _ => 0
            };
            return workflowNumber != 0;
        }
    }

    private sealed record Cursor(
        AttributeRanges Ranges,
        Workflow Workflow,
        int RuleIndex);
}
