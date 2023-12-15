namespace Day11CosmicExpansion;

internal sealed class Universe
{
    private readonly IReadOnlyList<Vector> galaxies;

    private IReadOnlyDictionary<long, int>? galaxyRowRanks;
    private IReadOnlyDictionary<long, int>? galaxyColumnRanks;

    private Universe(
        IReadOnlyList<Vector> galaxies)
    {
        this.galaxies = galaxies;
    }

    public long CalculateSumOfDistances(
        long expansionFactor)
    {
        RankGalaxySectors();

        long distanceSum = 0;
        for (int i = 0; i < galaxies.Count; i++)
        {
            var galaxyOne = galaxies[i];
            for (int j = i + 1; j < galaxies.Count; j++)
            {
                var galaxyTwo = galaxies[j];
                distanceSum += CalculateDistanceInOneDimension(
                    galaxyOne.Row, galaxyTwo.Row, galaxyRowRanks!, expansionFactor);
                distanceSum += CalculateDistanceInOneDimension(
                    galaxyOne.Column, galaxyTwo.Column, galaxyColumnRanks!, expansionFactor);
            }
        }

        return distanceSum;
    }

    public static Universe LoadFrom(
        string path)
    {
        var galaxies = new List<Vector>();
        var allLines = File.ReadAllLines(path);
        for (int row = 0; row < allLines.Length; row++)
        {
            var line = allLines[row];
            for (int column = 0; column < line.Length; column++)
            {
                if (line[column] != '#')
                {
                    continue;
                }

                galaxies.Add(new Vector(row, column));
            }
        }

        return new Universe(galaxies);
    }

    private static long CalculateDistanceInOneDimension(
        long sectorOne,
        long sectorTwo,
        IReadOnlyDictionary<long, int> galaxySectorsRanks,
        long expansionFactor)
    {
        long distance = Math.Abs(sectorOne - sectorTwo);
        if (distance > 1)
        {
            int rankOne = galaxySectorsRanks[sectorOne];
            int rankTwo = galaxySectorsRanks[sectorTwo];
            int rankDifference = Math.Abs(rankTwo - rankOne);
            long expandingSectorCount = distance - rankDifference;
            distance += expandingSectorCount * (expansionFactor - 1);
        }
        return distance;
    }

    private void RankGalaxySectors()
    {
        if (galaxyRowRanks != null)
        {
            return;
        }

        var galaxyRowSet = new HashSet<long>(galaxies.Count);
        var galaxyColumnSet = new HashSet<long>(galaxies.Count);
        foreach (var galaxy in galaxies)
        {
            galaxyRowSet.Add(galaxy.Row);
            galaxyColumnSet.Add(galaxy.Column);
        }

        galaxyRowRanks = ToRankMap(galaxyRowSet);
        galaxyColumnRanks = ToRankMap(galaxyColumnSet);

        static Dictionary<long, int> ToRankMap(
            HashSet<long> sectorSet)
        {
            int i = 0;
            var map = new Dictionary<long, int>(sectorSet.Count);
            foreach (var sector in sectorSet.OrderBy(s => s))
            {
                map.Add(sector, i++);
            }
            return map;
        }
    }
}
