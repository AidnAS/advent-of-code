from functools import reduce


def part1(data):
    counts = []
    for time, record in zip(
        *map(
            lambda s: map(int, s), map(lambda s: s.split(":")[1].strip().split(), data)
        )
    ):
        counts.append(int((time**2 - 4 * record) ** 0.5) + 1)
    return reduce(int.__mul__, counts)


def part2(data):
    time, record = map(
        int, map(lambda s: s.split(":")[1].strip().replace(" ", ""), data)
    )
    return int((time**2 - 4 * record) ** 0.5) + 1


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    return part1(data), part2(data)


def main():
    print(solve("aoc/six/input.txt"))


if __name__ == "__main__":
    main()
