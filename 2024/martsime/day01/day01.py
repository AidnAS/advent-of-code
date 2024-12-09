with open("input.txt") as f:
    lines = f.read().splitlines()


def parse():
    values = [[int(x) for x in line.split()] for line in lines]
    left, right = [], []
    for pair in values:
        left.append(pair[0])
        right.append(pair[1])
    return left, right


def part01():
    total = 0
    left, right = parse()

    left.sort()
    right.sort()

    for i in range(len(left)):
        total += abs(left[i] - right[i])
    print(f"Part 01: {total}")


def part02():
    total = 0
    left, right = parse()

    counts = {}
    for value in right:
        if value in counts:
            counts[value] += 1
        else:
            counts[value] = 1

    for value in left:
        total += value * counts.get(value, 0)
    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
