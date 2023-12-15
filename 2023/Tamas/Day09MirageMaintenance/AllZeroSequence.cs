namespace Day09MirageMaintenance;

internal sealed class AllZeroSequence : Sequence
{
    public static readonly AllZeroSequence Instance = new();

    public override int ExtrapolateNextNumber()
    {
        return 0;
    }

    public override int ExtrapolatePreviousNumber()
    {
        return 0;
    }
}
