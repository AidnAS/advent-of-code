from functools import reduce


def parse(d):
    return list(
        map(
            lambda g: len(list(filter(lambda c: c in g[0], g[1]))),
            map(
                lambda k: tuple(i.strip().split() for i in k),
                map(lambda m: m.split(":")[1].split("|"), d),
            ),
        )
    )


def part1(m):
    return sum(map(lambda s: 2 ** (s - 1), filter(None, m)))


def part2(m):
    return sum(
        reduce(
            lambda a, v: [*a, (1 + sum(a[v[0] - v[1] : v[0]]))],  # noqa: E203
            enumerate(reversed(m)),
            [],
        )
    )


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    parsed = parse(data)
    return part1(parsed), part2(parsed)


def main():
    print(solve("aoc/four/input.txt"))


if __name__ == "__main__":
    main()
