import math


def parse(lines):
    instructions, *lines = lines
    instructions = [v == "R" for v in instructions.strip()]
    graph = {}
    for line in lines[1:]:
        node, left, right = line[0:3], line[7:10], line[12:15]
        graph[node] = (left, right)
    return instructions, graph


def track2end(instructions, graph, start, end="Z"):
    cur, i, m = start, 0, len(instructions)
    while not cur.endswith(end):
        cur = graph[cur][instructions[i % m]]
        i += 1
    return i


def part1(inst, graph):
    return track2end(inst, graph, "AAA", end="ZZZ")


def part2(instructions, graph):
    ret = 1
    for node in graph:
        if node.endswith("A"):
            ret = math.lcm(ret, track2end(instructions, graph, node))
    return ret


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    instructions, graph = parse(data)
    return part1(instructions, graph), part2(instructions, graph)


def main():
    print(solve("aoc/eight/input.txt"))


if __name__ == "__main__":
    main()
