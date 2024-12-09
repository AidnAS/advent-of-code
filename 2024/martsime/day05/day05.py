with open("input.txt") as f:
    lines = f.read().splitlines()


def parse():
    rules = {}
    pages = []
    for line in lines:
        if "|" in line:
            before, after = [int(x) for x in line.split("|")]

            if before in rules:
                rules[before].append(after)
            else:
                rules[before] = [after]

        if "," in line:
            pages.append([int(x) for x in line.split(",")])

    return rules, pages


def validate(rules, pages):
    printed = set()
    for page in pages:
        for after in rules.get(page, []):
            if after in printed:
                return False, page, after

        printed.add(page)

    return True, None, None


def part01():
    total = 0
    rules, orderings = parse()

    for ordering in orderings:
        possible, _, _ = validate(rules, ordering)
        if possible:
            total += ordering[len(ordering) // 2]

    print(f"Part 01: {total}")


def part02():
    total = 0
    rules, orderings = parse()

    for ordering in orderings:
        possible, page, after = validate(rules, ordering)
        if not possible:
            while not possible:
                page_index = ordering.index(page)
                after_index = ordering.index(after)
                ordering[page_index] = after
                ordering[after_index] = page
                possible, page, after = validate(rules, ordering)

            assert possible
            total += ordering[len(ordering) // 2]

    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
