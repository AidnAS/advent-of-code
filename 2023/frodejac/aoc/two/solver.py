from functools import reduce


def p1(a):
    return sum(
        (
            int(b[5:])
            for b, c in map(lambda d: d.split(":"), a)
            if all(
                map(
                    lambda e: all(
                        map(
                            lambda f: (f[1][0] == "r" and int(f[0]) <= 12)
                            or (f[1][0] == "b" and int(f[0]) <= 13)
                            or (f[1][0] == "g" and int(f[0]) <= 14),
                            (g.split() for g in e.split(",")),
                        )
                    ),
                    c.split(";"),
                )
            )
        )
    )


def part2(a):
    return sum(
        reduce(lambda b, c: b * c, d.values())
        for d in map(
            lambda e: reduce(
                lambda f, g: {**f, g[1][0]: max(f[g[1][0]], int(g[0]))},
                (h.split() for i in e.split(";") for h in i.split(",")),
                {"r": 0, "g": 0, "b": 0},
            ),
            (j.split(":")[1] for j in a),
        )
    )


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    return p1(data), part2(data)


def main():
    print(solve("aoc/two/input.txt"))


if __name__ == "__main__":
    main()
