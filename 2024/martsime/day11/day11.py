with open("input.txt") as f:
    lines = f.read().splitlines()


def part01():
    stones = [int(x) for x in lines[0].split(" ")]

    iterations = 25

    for _ in range(iterations):
        new_stones = []
        for stone in stones:
            if stone == 0:
                new_stones.append(1)
                continue
            elif len(str(stone)) % 2 == 0:
                s = str(stone)
                left = s[: len(s) // 2]
                right = s[len(s) // 2 :]
                new_stones.extend([int(left), int(right)])
            else:
                new_stones.append(stone * 2024)

        stones = new_stones

    print(f"Part 01: {len(stones)}")


def split_stone(stone):
    stones = {}
    if stone == 0:
        stones[1] = stones.get(1, 0) + 1
    elif len(str(stone)) % 2 == 0:
        s = str(stone)
        left = int(s[: len(s) // 2])
        right = int(s[len(s) // 2 :])
        stones[left] = stones.get(left, 0) + 1
        stones[right] = stones.get(right, 0) + 1
    else:
        new_stone = stone * 2024
        stones[new_stone] = stones.get(new_stone, 0) + 1

    return stones


def part02():
    stones = [int(x) for x in lines[0].split(" ")]
    stones = {stone: 1 for stone in stones}

    iterations = 75

    for _ in range(iterations):
        new_stones = {}
        for stone, count in stones.items():
            new = split_stone(stone)

            for new_stone, new_count in new.items():
                new_stones[new_stone] = new_stones.get(new_stone, 0) + new_count * count

        stones = new_stones

    print(f"Part 02: {sum(stones.values())}")


if __name__ == "__main__":
    part01()
    part02()
