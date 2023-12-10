namespace Day09MirageMaintenance;

internal sealed class RealSequence : Sequence
{
    private readonly IReadOnlyList<int> numbers;
    private readonly Sequence forwardDifference;

    private RealSequence(
        IReadOnlyList<int> numbers)
    {
        this.numbers = numbers;
        forwardDifference = CalculateForwardDifference();
    }

    public override int ExtrapolateNextNumber()
    {
        return numbers[^1] + forwardDifference.ExtrapolateNextNumber();
    }

    public override int ExtrapolatePreviousNumber()
    {
        return numbers[0] - forwardDifference.ExtrapolatePreviousNumber();
    }

    public static RealSequence Parse(
        string line)
    {
        var numbers = line
            .Split(' ')
            .Select(int.Parse)
            .ToList();
        return new RealSequence(numbers);
    }

    private Sequence CalculateForwardDifference()
    {
        int forwardDifferenceCount = numbers.Count - 1;
        var forwardDifferenceNumbers = new int[forwardDifferenceCount];
        bool isAllZeros = true;
        for (int i = 0; i < forwardDifferenceCount; i++)
        {
            int difference = numbers[i + 1] - numbers[i];
            forwardDifferenceNumbers[i] = difference;
            isAllZeros &= difference == 0;
        }

        return isAllZeros
            ? AllZeroSequence.Instance
            : new RealSequence(forwardDifferenceNumbers);
    }
}
