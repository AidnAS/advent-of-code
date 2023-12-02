from functools import reduce


def parse_line_2(line):
    header, game = line.split(":")
    return int(header[5:]), game


def part1(lines):
    return sum(
        (
            int(id_[5:])
            for id_, game in map(lambda line: line.split(":"), lines)
            if all(
                map(
                    lambda hand: all(
                        map(
                            lambda d: (d[1] == "red" and int(d[0]) <= 12)
                            or (d[1] == "blue" and int(d[0]) <= 13)
                            or (d[1] == "green" and int(d[0]) <= 14),
                            (draw.split() for draw in hand.split(",")),
                        ),
                    ),
                    game.split(";"),
                )
            )
        )
    )


def part2(lines):
    return sum(
        reduce(lambda acc, v: acc * v, cubemax.values())
        for cubemax in map(
            lambda game: reduce(
                lambda acc, cube: {**acc, cube[1]: max(acc[cube[1]], int(cube[0]))},
                (cube.split() for h in game.split(";") for cube in h.split(",")),
                {"red": 0, "green": 0, "blue": 0},
            ),
            (line.split(":")[1] for line in lines),
        )
    )


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    return part1(data), part2(data)


def main():
    print(solve("aoc/two/input.txt"))


if __name__ == "__main__":
    main()
