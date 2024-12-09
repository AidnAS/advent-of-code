with open("input.txt") as f:
    lines = f.read().splitlines()


def part01():
    board = [list(row) for row in lines]
    height = len(board)
    width = len(board[0])

    towers = {}

    for y in range(height):
        for x in range(width):
            cell = board[y][x]
            if cell != ".":
                if cell not in towers:
                    towers[cell] = [(y, x)]
                else:
                    towers[cell].append((y, x))

    for values in towers.values():
        for y1, x1 in values:
            for y2, x2 in values:
                if y1 == y2 and x1 == x2:
                    continue

                vy, vx = (y2 - y1, x2 - x1)
                y, x = (y2, x2)

                y += vy
                x += vx

                if y < 0 or y >= height or x < 0 or x >= width:
                    continue

                board[y][x] = "#"

    for row in board:
        print("".join(row))

    total = 0

    for row in board:
        for cell in row:
            if cell == "#":
                total += 1
    print(f"Part 01: {total}")


def part02():
    board = [list(row) for row in lines]
    height = len(board)
    width = len(board[0])

    towers = {}

    for y in range(height):
        for x in range(width):
            cell = board[y][x]
            if cell != ".":
                if cell not in towers:
                    towers[cell] = [(y, x)]
                else:
                    towers[cell].append((y, x))

    for values in towers.values():
        for y1, x1 in values:
            for y2, x2 in values:
                if y1 == y2 and x1 == x2:
                    continue

                vy, vx = (y2 - y1, x2 - x1)
                y, x = (y2, x2)

                while True:
                    y += vy
                    x += vx

                    if y < 0 or y >= height or x < 0 or x >= width:
                        break

                    board[y][x] = "#"

                    if len(values) >= 2:
                        board[y2][x2] = "#"
                        board[y1][x1] = "#"

    for row in board:
        print("".join(row))

    total = 0

    for row in board:
        for cell in row:
            if cell == "#":
                total += 1
    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
