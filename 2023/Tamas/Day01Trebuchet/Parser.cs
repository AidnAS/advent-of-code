namespace Day01Trebuchet;

internal sealed class Parser
{
    private readonly State initialState;

    private Parser(
        State initialState)
    {
        this.initialState = initialState;
    }

    public static Parser LoadRulesFrom(string fileName)
    {
        var allLines = File.ReadAllLines(fileName);

        var initialState = new State();
        var stateMap = new Dictionary<string, State>();
        foreach (var line in allLines)
        {
            var parts = line.Split('=');
            if (parts.Length < 2)
            {
                continue;
            }

            var digit = int.Parse(parts[1]);
            var nextState = new State(digit);
            var symbol = parts[0];
            stateMap.Add(symbol, nextState);

            for (int i = symbol.Length - 1; i > 0; i--)
            {
                string prefix = symbol[..i];
                if (!stateMap.TryGetValue(prefix, out var state))
                {
                    state = new State();
                    stateMap.Add(prefix, state);
                }

                state.NextMap.Add(symbol[i], nextState);
                nextState = state;
            }

            initialState.NextMap[symbol[0]] = nextState;
        }

        return new Parser(initialState);
    }

    public int GetNumberFromFirstAndLastDigit(
        string line)
    {
        return
            10 * GetFirstDigit(line, Direction.Forward) 
            + GetFirstDigit(line, Direction.Backward);
    }

    private int GetFirstDigit(
        string input,
        Direction direction)
    {
        for (int step = 0; step < input.Length; step++)
        {
            int globalIndex = direction == Direction.Forward
                ? step
                : input.Length - 1 - step;
            
            var state = initialState;
            for (int localIndex = globalIndex; localIndex < input.Length; localIndex++)
            {
                var token = input[localIndex];
                if (!state.NextMap.TryGetValue(token, out state))
                {
                    break;
                }

                if (state.Digit.HasValue)
                {
                    return state.Digit.Value;
                }
            }
        }

        return 0;
    }

    private enum Direction
    {
        Forward,
        Backward
    }

    private sealed class State
    {
        public State(
            int? digit = null)
        {
            Digit = digit;
            NextMap = new();
        }

        public int? Digit { get; }
        public Dictionary<char, State> NextMap { get; }
    }
}
