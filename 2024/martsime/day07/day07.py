from itertools import product

with open("input.txt") as f:
    lines = f.read().splitlines()


def parse_values():
    return [[int(x) for x in row.replace(":", "").split(" ")] for row in lines]


def part01():
    total = 0

    for row in parse_values():
        lhs = row[0]
        rhs = row[1:]

        num_signs = len(rhs) - 1
        combinations = 2**num_signs

        for i in range(1, combinations + 1):
            signs = []
            for j in range(1, num_signs + 1):
                if i & (1 << num_signs - j):
                    signs.append("+")
                else:
                    signs.append("*")

            value = rhs[0]
            for j in range(num_signs):
                if signs[j] == "+":
                    value += rhs[j + 1]
                else:
                    value *= rhs[j + 1]

            if value == lhs:
                total += lhs
                break

    print(f"Part 01: {total}")


def part02():
    total = 0
    for row in parse_values():
        lhs = row[0]
        rhs = row[1:]

        num_signs = len(rhs) - 1
        products = product(["+", "*", "|"], repeat=num_signs)

        for p in products:
            value = rhs[0]
            for i in range(num_signs):
                if p[i] == "+":
                    value += rhs[i + 1]
                elif p[i] == "*":
                    value *= rhs[i + 1]
                else:
                    value = int(f"{value}{rhs[i+1]}")

            if value == lhs:
                total += lhs
                break

    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
