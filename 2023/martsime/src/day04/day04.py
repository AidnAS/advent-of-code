with open("input.txt", "r") as f:
    lines = f.readlines()


def part1():
    total = 0
    for line in lines:
        _, cards = line.split(": ")
        winning_cards, my_cards = cards.split(" | ")
        winning_cards = set([int(x) for x in winning_cards.split(" ") if x != ""])
        my_cards = set([int(x) for x in my_cards.split(" ") if x != ""])
        matches = len(winning_cards.intersection(my_cards))
        score = 0
        if matches > 0:
            score = 2 ** (matches - 1)
        total += score
    print(f"Part 1: {total}")


def part2():
    total = 0
    multipliers = [1] * len(lines)
    for card, line in enumerate(lines):
        _, cards = line.split(": ")
        winning_cards, my_cards = cards.split(" | ")
        winning_cards = set([int(x) for x in winning_cards.split(" ") if x != ""])
        my_cards = set([int(x) for x in my_cards.split(" ") if x != ""])
        matches = len(winning_cards.intersection(my_cards))
        m = multipliers[card]
        total += m
        # print(f"Card {card + 1} gives {m} points")
        for i in range(matches):
            next_card = card + i + 1
            if next_card < len(lines):
                multipliers[next_card] += m
            else:
                total += m * matches
    print(f"Part 2: {total}")


if __name__ == "__main__":
    part1()
    part2()
