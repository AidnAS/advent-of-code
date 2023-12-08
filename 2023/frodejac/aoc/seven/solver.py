from collections import Counter
from functools import cmp_to_key

card_rank = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"]


def parse(lines, jacks_wild=False):
    hands = []
    for line in lines:
        hand, bid = line.split()
        most_common = Counter(hand).most_common(2)
        if jacks_wild:
            if (c := most_common[0][0]) != "J":
                most_common = Counter(hand.replace("J", c)).most_common(2)
            elif hand != "JJJJJ":
                most_common = Counter(hand.replace("J", most_common[1][0])).most_common(
                    2
                )
            else:
                pass

        match most_common:
            case [(_, 5), *_]:
                hands.append((hand, int(bid), 6))
            case [(_, 4), *_]:
                hands.append((hand, int(bid), 5))
            case [(_, 3), (_, 2)]:
                hands.append((hand, int(bid), 4))
            case [(_, 3), (_, 1)]:
                hands.append((hand, int(bid), 3))
            case [(_, 2), (_, 2)]:
                hands.append((hand, int(bid), 2))
            case [(_, 2), (_, 1)]:
                hands.append((hand, int(bid), 1))
            case [(_, 1), _]:
                hands.append((hand, int(bid), 0))
    return hands


def order(jacks_wild=False):
    def _order(a, b):
        if a[-1] != b[-1]:
            if a[2] > b[2]:
                return 1
            return -1
        for d, e in zip(a[0], b[0]):
            if d != e:
                rank_d = (
                    0 if d == "J" and jacks_wild else card_rank.index(d) + jacks_wild
                )
                rank_e = (
                    0 if e == "J" and jacks_wild else card_rank.index(e) + jacks_wild
                )
                if rank_d > rank_e:
                    return 1
                return -1
        return 0

    return _order


def part1(lines):
    hands = parse(lines)
    return sum(
        (
            (i + 1) * j[1]
            for i, j in enumerate(
                sorted(
                    hands,
                    key=cmp_to_key(order()),
                )
            )
        )
    )


def part2(lines):
    hands = parse(lines, jacks_wild=True)
    return sum(
        (
            (i + 1) * j[1]
            for i, j in enumerate(
                sorted(
                    hands,
                    key=cmp_to_key(order(jacks_wild=True)),  # type: ignore
                )
            )
        )
    )


def solve(inputpath):
    with open(inputpath) as f:
        data = f.readlines()
    return part1(data), part2(data)


def main():
    print(solve("aoc/seven/input.txt"))


if __name__ == "__main__":
    main()
