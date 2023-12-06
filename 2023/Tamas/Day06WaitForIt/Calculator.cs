namespace Day06WaitForIt;

internal static class Calculator
{
    public static long FindNumberOfWaysToWin(
        Race race)
    {
        double underSqrt = Math.Pow(race.TimeLimit, 2) - 4 * race.RecordDistance;
        if (underSqrt <= 0)
        {
            return 0;
        }

        double sqrt = Math.Sqrt(underSqrt);
        double mind = (race.TimeLimit - sqrt) / 2;
        double maxd = (race.TimeLimit + sqrt) / 2;
        double mindCeiling = Math.Ceiling(mind);
        double maxdFloor = Math.Floor(maxd);
        // Fishy equality check with doubles :s
        long min = (long)mindCeiling + (mind == mindCeiling ? 1 : 0);
        long max = (long)maxdFloor - (maxd == maxdFloor ? 1 : 0);
        if (min >= max)
        {
            return 0;
        }

        return max - min + 1;
    }
}
