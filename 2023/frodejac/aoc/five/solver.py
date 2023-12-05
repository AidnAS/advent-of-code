def parse(lines):
    seeds, *lines = list(filter(None, map(str.strip, lines)))
    seeds = list(map(int, seeds.split(":")[1].split()))

    maps = []
    for line in lines:
        if "map" in line:
            maps.append([])
        else:
            dst, src, range_ = map(int, line.split())
            maps[-1].append((dst, src, range_))

    return seeds, [sorted(m, key=lambda v: v[1]) for m in maps]


def map_(maps, ranges):
    for m in maps:
        done = []
        for r in ranges:
            remaining = [r]
            for dst, src, r2 in m:
                if not remaining:
                    break
                (start, r1), *remaining = remaining
                s1, e1 = start, start + r1 - 1
                s2, e2 = src, src + r2 - 1

                a = (s1, min(e1, s2 - 1))  # before intersection
                b = (max(s1, s2), min(e1, e2))  # intersection
                c = (max(s1, e2 + 1), e1)  # after intersection

                if (o := a[1] - a[0] + 1) > 0:
                    remaining.append((a[0], o))
                if (o := b[1] - b[0] + 1) > 0:
                    done.append((b[0] + (dst - src), o))
                if (o := c[1] - c[0] + 1) > 0:
                    remaining.append((c[0], o))
            done += remaining
        ranges = done
    return ranges


def part1(seeds, maps):
    ranges = map(lambda s: (s, 1), seeds)
    locations = (v[0] for v in map_(maps, ranges))
    return min(locations)


def part2(seeds, maps):
    ranges = zip(seeds[::2], seeds[1::2])
    locations = (v[0] for v in map_(maps, ranges))
    return min(locations)


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    parsed = parse(data)
    return part1(*parsed), part2(*parsed)


def main():
    print(solve("aoc/five/input.txt"))


if __name__ == "__main__":
    main()
