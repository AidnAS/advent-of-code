import re


def part1(lines):
    total, first, last = 0, 0, 0
    for line in lines:
        for c in line:
            if c in "123456789":
                if first == 0:
                    first = c
                last = c
        total += int(first + last)
        first, last = 0, 0
    return total


def part2(lines):
    def convert(s):
        match s:
            case "1" | "one" | "eno":
                return 1
            case "2" | "two" | "owt":
                return 2
            case "3" | "three" | "eerht":
                return 3
            case "4" | "four" | "ruof":
                return 4
            case "5" | "five" | "evif":
                return 5
            case "6" | "six" | "xis":
                return 6
            case "7" | "seven" | "neves":
                return 7
            case "8" | "eight" | "thgie":
                return 8
            case "9" | "nine" | "enin":
                return 9
            case x:
                raise Exception(f"unknown value: {x}")

    p1 = re.compile(
        r"([1-9])|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)"
    )
    p2 = re.compile(
        r"([1-9])|(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)"
    )
    total = 0
    for line in lines:
        m1, m2 = re.search(p1, line), re.search(p2, line[::-1])
        if m1 is None or m2 is None:
            raise Exception("This shouldn't happen")

        total += convert(m1[0]) * 10 + convert(m2[0])
    return total


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    return part1(data), part2(data)


def main():
    print(solve("aoc/one/input.txt"))


if __name__ == "__main__":
    main()
