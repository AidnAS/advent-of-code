with open("input.txt") as f:
    lines = f.read().splitlines()


def part01():
    total = 0
    values = [int(x) for x in lines[0]]

    index = 0
    blocks = []

    for i in range(len(values)):
        if i % 2 == 0:
            blocks.extend([str(index)] * values[i])
            index += 1
        else:
            blocks.extend(["."] * values[i])

    last_index = len(blocks) - 1
    first_empty_block_index = 0
    while first_empty_block_index <= last_index:
        while blocks[last_index] == ".":
            last_index -= 1
            continue

        while blocks[first_empty_block_index] != ".":
            first_empty_block_index += 1
            continue

        if first_empty_block_index > last_index:
            break

        last_block = blocks[last_index]
        blocks[last_index] = "."
        index = first_empty_block_index
        blocks[index] = last_block

    for i in range(len(blocks)):
        if blocks[i] != ".":
            total += i * int(blocks[i])

    print(f"Part 01: {total}")


def part02():
    total = 0
    values = [int(x) for x in lines[0]]

    index = 0
    blocks = []

    for i in range(len(values)):
        if i % 2 == 0:
            blocks.extend([str(index)] * values[i])
            index += 1
        else:
            blocks.extend(["."] * values[i])

    last_index = len(blocks) - 1
    while last_index >= 0:
        while blocks[last_index] == ".":
            last_index -= 1
            continue

        block_size = 0
        block_index = last_index
        block_value = blocks[block_index]

        while blocks[block_index] == block_value:
            block_index -= 1
            block_size += 1

        for index in range(block_index):
            if blocks[index] != ".":
                continue

            if blocks[index : index + block_size] != ["."] * block_size:
                continue

            for i in range(block_size):
                blocks[index + i] = block_value
                blocks[block_index + 1 + i] = "."

            break

        last_index = block_index

    for i in range(len(blocks)):
        if blocks[i] != ".":
            total += i * int(blocks[i])
    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
