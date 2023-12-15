def parse(lines):
    return [[*line.strip()] for line in lines]


def offsets(m, expansion_rate=2):
    ex, ey = [], []
    e = 0
    for row in m:
        ey.append(e)
        e += (expansion_rate - 1) * all(c == "." for c in row)
    e = 0
    for x in range(len(m[0])):
        ex.append(e)
        e += (expansion_rate - 1) * all(m[y][x] == "." for y in range(len(m)))
    return ex, ey


def find_galaxies(m, ex, ey):
    galaxies = []
    for y in range(len(m)):
        for x in range(len(m[0])):
            if m[y][x] == "#":
                galaxies.append((x + ex[x], y + ey[y]))
    return galaxies


def pairwise_dist(g):
    for i in range(len(g)):
        for j in range(i + 1, len(g)):
            x1, y1 = g[i]
            x2, y2 = g[j]
            yield abs(x1 - x2) + abs(y1 - y2)


def part1(lines):
    m = parse(lines)
    ex, ey = offsets(m, 2)
    galaxies = find_galaxies(m, ex, ey)
    return sum(pairwise_dist(galaxies))


def part2(lines):
    m = parse(lines)
    ex, ey = offsets(m, 1000000)
    galaxies = find_galaxies(m, ex, ey)
    return sum(pairwise_dist(galaxies))


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    return part1(data), part2(data)


def main():
    print(solve("aoc/eleven/input.txt"))


if __name__ == "__main__":
    main()
