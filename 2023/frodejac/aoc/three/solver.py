adjacency = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
]


def parse(lines):
    parts, numbers, lut = {}, {}, []
    for i, line in enumerate(lines):
        num, start = "", None
        for j, c in enumerate(line):
            if c in "0123456789":
                if start is None:
                    start = j
                num += c
            else:
                if start is not None:
                    lut.append(int(num))
                    id_ = len(lut) - 1
                    for k in range(start, j):
                        numbers[(i, k)] = id_
                    num, start = "", None
                if c not in ".\n":
                    parts[(i, j)] = c
    return parts, numbers, lut


def part1(parts, numbers, lut):
    adjacent = set()
    for x, y in parts:
        for i, j in adjacency:
            if (v := numbers.get((x + i, y + j))) is not None:
                adjacent.add(v)
    return sum(lut[id_] for id_ in adjacent)


def part2(parts, numbers, lut):
    total = 0
    for (x, y), part in parts.items():
        if part == "*":
            adjacent = set()
            for i, j in adjacency:
                if (v := numbers.get((x + i, y + j))) is not None:
                    adjacent.add(v)
            if len(adjacent) == 2:
                a, b = tuple(adjacent)
                total += lut[a] * lut[b]
    return total


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    parsed = parse(data)
    return part1(*parsed), part2(*parsed)


def main():
    print(solve("aoc/three/input.txt"))


if __name__ == "__main__":
    main()
