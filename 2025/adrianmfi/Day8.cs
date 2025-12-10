

using System.Collections;
using System.Collections.Immutable;
using System.Reflection.Metadata.Ecma335;
using System.Text.RegularExpressions;

namespace adrianmfi;

public static partial class Day8
{
    static readonly string test = """
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
    """;


    public static long Part1()
    {
        List<Position> positions = [];
        // var numIterations = 10;
        // foreach (var line in test.Split('\n'))
        var numIterations = 1000;
        foreach (var line in File.ReadLines("inputs/day8.txt"))
        {
            var split = line.Split(",").Select(int.Parse).ToArray();
            positions.Add(new(split[0], split[1], split[2]));
        }
        List<(Position position1, Position position2, double distance)> distances = [];
        for (int i = 0; i < positions.Count - 1; i++)
        {
            var positionI = positions[i];
            for (int j = i + 1; j < positions.Count; j++)
            {
                var positionJ = positions[j];
                distances.Add((positionI, positionJ, positionI.Distance(positionJ)));
            }
        }
        distances.Sort((d1, d2) => d1.distance.CompareTo(d2.distance));

        Dictionary<Position, HashSet<Position>> circuits = [];
        for (int i = 0; i < numIterations; i++)
        {
            var (position1, position2, distance) = distances[i];
            circuits.TryGetValue(position1, out var existingCircuitForP1);
            circuits.TryGetValue(position2, out var existingCircuitForP2);
            existingCircuitForP1 ??= [position1];
            existingCircuitForP2 ??= [position2];
            existingCircuitForP1.UnionWith(existingCircuitForP2);
            foreach (var item in existingCircuitForP1)
            {
                circuits[item] = existingCircuitForP1;
            }
        }
        var circuitSizes = circuits.Values.Distinct().Select(circuit => circuit.Count).ToList();
        circuitSizes.Sort();
        return circuitSizes.TakeLast(3).Aggregate((a, b) => a * b);
    }



    public static long Part2()
    {
        List<Position> positions = [];
        // foreach (var line in test.Split('\n'))
        foreach (var line in File.ReadLines("inputs/day8.txt"))
        {
            var split = line.Split(",").Select(int.Parse).ToArray();
            positions.Add(new(split[0], split[1], split[2]));
        }
        List<(Position position1, Position position2, double distance)> distances = [];
        for (int i = 0; i < positions.Count - 1; i++)
        {
            var positionI = positions[i];
            for (int j = i + 1; j < positions.Count; j++)
            {
                var positionJ = positions[j];
                distances.Add((positionI, positionJ, positionI.Distance(positionJ)));
            }
        }
        distances.Sort((d1, d2) => d1.distance.CompareTo(d2.distance));

        Dictionary<Position, HashSet<Position>> circuits = [];
        int distanceIterator = 0;
        var circuitCount = 0;
        while (circuitCount < positions.Count)
        {
            var (position1, position2, distance) = distances[distanceIterator];
            circuits.TryGetValue(position1, out var existingCircuitForP1);
            circuits.TryGetValue(position2, out var existingCircuitForP2);
            existingCircuitForP1 ??= [position1];
            existingCircuitForP2 ??= [position2];
            existingCircuitForP1.UnionWith(existingCircuitForP2);
            foreach (var item in existingCircuitForP1)
            {
                circuits[item] = existingCircuitForP1;
            }
            circuitCount = existingCircuitForP1.Count;
            distanceIterator++;
        }

        var finalPosition = distances[distanceIterator - 1];
        return finalPosition.position1.X * finalPosition.position2.X;
    }

    record Position(long X, long Y, long Z)
    {
        public double Distance(Position other)
        {
            var x = X - other.X;
            var y = Y - other.Y;
            var z = Z - other.Z;
            return Math.Sqrt(x * x + y * y + z * z);
        }
    }

}
