import re

with open("input.txt") as f:
    lines = f.read().splitlines()


def part01():
    total = 0
    for line in lines:
        for match in re.findall(r"mul\(\d+,\d+\)", line):
            values = [int(x) for x in match.lstrip("mul(").rstrip(")").split(",")]
            total += values[0] * values[1]

    print(f"Part 01: {total}")


def part02():
    total = 0
    do = True
    for line in lines:
        multiplications = re.findall(r"mul\(\d+,\d+\)", line)
        index = 0
        while len(multiplications) > 0:
            line = line[index:]
            multiplication_index = line.find(multiplications[0])
            do_index = line.find("do()")
            dont_index = line.find("don't()")

            indexes = list(
                filter(lambda x: x >= 0, [multiplication_index, do_index, dont_index])
            )
            indexes.sort()

            first_index = indexes[0]
            if first_index == multiplication_index:
                multiplication = multiplications.pop(0)
                if do:
                    values = [
                        int(x)
                        for x in multiplication.lstrip("mul(").rstrip(")").split(",")
                    ]
                    total += values[0] * values[1]
            elif first_index == do_index:
                do = True
            elif first_index == dont_index:
                do = False

            index = first_index + 1

    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
