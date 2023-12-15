def parse(lines):
    def s(x1, y1, x2, y2):
        return x1 + x2, y1 + y2

    north, south, east, west = (0, -1), (0, 1), (1, 0), (-1, 0)
    start, graph = None, {}
    for y, line in enumerate(lines):
        for x, c in enumerate(line.strip()):
            match c:
                case "|":
                    graph[(x, y)] = (s(x, y, *north), s(x, y, *south))
                case "-":
                    graph[(x, y)] = (s(x, y, *east), s(x, y, *west))
                case "L":
                    graph[(x, y)] = (s(x, y, *north), s(x, y, *east))
                case "J":
                    graph[(x, y)] = (s(x, y, *north), s(x, y, *west))
                case "7":
                    graph[(x, y)] = (s(x, y, *south), s(x, y, *west))
                case "F":
                    graph[(x, y)] = (s(x, y, *south), s(x, y, *east))
                case ".":
                    continue
                case "S":
                    edges = []
                    if lines[y + north[1]][x + north[0]] in ("|", "7", "F"):
                        edges.append(north)
                    if lines[y + east[1]][x + east[0]] in ("-", "J", "7"):
                        edges.append(east)
                    if lines[y + south[1]][x + south[0]] in ("|", "L", "J"):
                        edges.append(south)
                    if lines[y + west[1]][x + west[0]] in ("-", "L", "F"):
                        edges.append(west)
                    graph[(x, y)] = (s(x, y, *edges[0]), s(x, y, *edges[1]))
                    start = (x, y)
    return graph, start


def next_(graph, prev, node):
    a, b = graph[node]
    if prev == a:
        return b
    return a


def part1(graph, start):
    steps = 1
    current, prev = start, None
    while (n := next_(graph, prev, current)) != start:
        steps += 1
        prev = current
        current = n
    return steps // 2


def part2(graph, start):
    visited = [start]
    circumference = 1
    current, prev = start, None
    while (n := next_(graph, prev, current)) != start:
        visited.append(n)
        circumference += 1
        prev = current
        current = n

    # shoelace
    area = 0
    m = len(visited)
    for i in range(m):
        x1, y1 = visited[i]
        x2, y2 = visited[(i + 1) % m]
        area += x1 * y2 - y1 * x2

    return (abs(area) - circumference) // 2 + 1


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    parsed = parse(data)
    return part1(*parsed), part2(*parsed)


def main():
    print(solve("aoc/ten/input.txt"))


if __name__ == "__main__":
    main()
