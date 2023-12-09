def parse(lines):
    return [list(map(int, line.split())) for line in lines]


def part1(sequences):
    extrapolated_sum = 0
    for sequence in sequences:
        diffs = [sequence]
        while any(diffs[-1]):
            d = []
            for i, j in zip(diffs[-1], diffs[-1][1:]):
                d.append(j - i)
            diffs.append(d)
        v = 0
        for diff in reversed(diffs[:-1]):
            v += diff[-1]
        extrapolated_sum += v
    return extrapolated_sum


def part2(sequences):
    extrapolated_sum = 0
    for sequence in sequences:
        diffs = [sequence]
        while any(diffs[-1]):
            d = []
            for i, j in zip(diffs[-1], diffs[-1][1:]):
                d.append(j - i)
            diffs.append(d)
        v = 0
        for diff in reversed(diffs[:-1]):
            v = diff[0] - v
        extrapolated_sum += v
    return extrapolated_sum


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    parsed = parse(data)
    return part1(parsed), part2(parsed)


def main():
    print(solve("aoc/nine/input.txt"))


if __name__ == "__main__":
    main()
