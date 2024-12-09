with open("input.txt") as f:
    lines = f.read().splitlines()

DIRECTIONS = [(-1, 0), (0, 1), (1, 0), (0, -1), (-1, -1), (-1, 1), (1, -1), (1, 1)]
WORD = "XMAS"


def check(board, height, width, y, x, direction):
    for step in range(1, len(WORD)):
        ny, nx = y + step * direction[0], x + step * direction[1]
        if ny < 0 or ny >= height or nx < 0 or nx >= width:
            return False
        if board[ny][nx] != WORD[step]:
            return False
    return True


def check_part02(board, height, width, y, x):
    valid = "MS"

    y1, x1 = y - 1, x - 1
    y2, x2 = y - 1, x + 1
    y3, x3 = y + 1, x + 1
    y4, x4 = y + 1, x - 1

    if y1 < 0 or x1 < 0 or y1 >= height or x1 >= width:
        return False
    if y2 < 0 or x2 < 0 or y2 >= height or x2 >= width:
        return False
    if y3 < 0 or x3 < 0 or y3 >= height or x3 >= width:
        return False
    if y4 < 0 or x4 < 0 or y4 >= height or x4 >= width:
        return False

    if (
        board[y1][x1] == board[y3][x3]
        or board[y1][x1] not in valid
        or board[y3][x3] not in valid
    ):
        return False
    if (
        board[y2][x2] == board[y4][x4]
        or board[y2][x2] not in valid
        or board[y4][x4] not in valid
    ):
        return False

    return True


def part01():
    total = 0
    board = [list(row) for row in lines]
    height, width = len(board), len(board[0])

    for y in range(height):
        for x in range(width):
            if board[y][x] == "X":
                for direction in DIRECTIONS:
                    if check(board, height, width, y, x, direction):
                        total += 1

    print(f"Part 01: {total}")


def part02():
    total = 0
    board = [list(row) for row in lines]
    height, width = len(board), len(board[0])

    for y in range(height):
        for x in range(width):
            if board[y][x] == "A":
                if check_part02(board, height, width, y, x):
                    total += 1

    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
