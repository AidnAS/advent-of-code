with open("input.txt") as f:
    lines = f.read().splitlines()


def validate(values):
    deltas = [values[i + 1] - values[i] for i in range(len(values) - 1)]

    all_positive = all(delta >= 1 and delta <= 3 for delta in deltas)
    all_negative = all(delta <= -1 and delta >= -3 for delta in deltas)

    if all_positive or all_negative:
        return True
    return False


def part01():
    total = 0
    reports = [[int(x) for x in line.split(" ")] for line in lines]

    for values in reports:
        if validate(values):
            total += 1

    print(f"Part 01: {total}")


def part02():
    total = 0
    reports = [[int(x) for x in line.split(" ")] for line in lines]

    for values in reports:
        if validate(values):
            total += 1
        else:
            for i in range(len(values)):
                if validate(values[:i] + values[i + 1 :]):
                    total += 1
                    break

    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
