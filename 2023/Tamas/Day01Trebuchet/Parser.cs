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
            10 * GetDigit(line, Position.First) 
            + GetDigit(line, Position.Last);
    }

    private int GetDigit(
        string input,
        Position position)
    {
        int digit = 0;
        var state = initialState;
        int currentMatchStart = -1;
        for (int i = 0; i < input.Length; i++)
        {
            var c = input[i];
            if (!state.NextMap.TryGetValue(c, out state))
            {
                if (currentMatchStart != -1)
                {
                    i = currentMatchStart;
                    currentMatchStart = -1;
                }

                state = initialState;
                continue;
            }

            if (currentMatchStart == -1)
            {
                currentMatchStart = i;
            }

            if (state.Digit.HasValue)
            {
                digit = state.Digit.Value;

                i = currentMatchStart;
                currentMatchStart = -1;
                state = initialState;

                if (position == Position.First)
                {
                    break;
                }
            }
        }

        return digit;
    }

    private enum Position
    {
        First,
        Last
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
