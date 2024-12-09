with open("input.txt") as f:
    lines = f.read().splitlines()

DIRECTIONS = [(-1, 0), (0, 1), (1, 0), (0, -1)]


def print_board(board, height, width, visited=None, loops=None):
    for y in range(height):
        for x in range(width):
            if (y, x) in board:
                print("#", end="")
            elif visited and (y, x) in visited:
                print("X", end="")
            elif loops and (y, x) in loops:
                print("O", end="")
            else:
                print(".", end="")
        print()


def part01():
    board = set()
    pos = (-1, -1)
    height, width = len(lines), len(lines[0])
    direction = 0
    for y, row in enumerate(lines):
        for x, cell in enumerate(row):
            if cell == "#":
                board.add((y, x))
            elif cell == "^":
                pos = (y, x)

    visited = set()

    while True:
        visited.add(pos)
        new_pos = (pos[0] + DIRECTIONS[direction][0], pos[1] + DIRECTIONS[direction][1])
        new_y, new_x = new_pos
        if new_y < 0 or new_y >= height or new_x < 0 or new_x >= width:
            break
        if new_pos in board:
            direction = (direction + 1) % 4
        new_pos = (pos[0] + DIRECTIONS[direction][0], pos[1] + DIRECTIONS[direction][1])
        pos = new_pos

    print_board(board, height, width, visited=visited)
    total = len(visited)
    print(f"Part 01: {total}")


def is_loop(pos, new_pos, start, direction, board, visited, height, width):
    if new_pos in board or new_pos == start:
        return False

    board = board.copy()
    visited = set()
    board.add(new_pos)
    pos = start
    direction = 0

    while True:
        visited.add((*pos, direction))
        new_pos = (pos[0] + DIRECTIONS[direction][0], pos[1] + DIRECTIONS[direction][1])
        new_y, new_x = new_pos
        if new_y < 0 or new_y >= height or new_x < 0 or new_x >= width:
            return False

        while new_pos in board:
            direction = (direction + 1) % 4
            new_pos = (
                pos[0] + DIRECTIONS[direction][0],
                pos[1] + DIRECTIONS[direction][1],
            )

        if (*new_pos, direction) in visited:
            return True
        pos = new_pos


def part02():
    board = set()
    start = (-1, -1)
    height, width = len(lines), len(lines[0])
    direction = 0
    for y, row in enumerate(lines):
        for x, cell in enumerate(row):
            if cell == "#":
                board.add((y, x))
            elif cell == "^":
                start = (y, x)

    pos = start
    visited = set()
    total = 0
    loops = set()

    while True:
        visited.add((*pos, direction))
        new_pos = (pos[0] + DIRECTIONS[direction][0], pos[1] + DIRECTIONS[direction][1])
        new_y, new_x = new_pos
        if new_y < 0 or new_y >= height or new_x < 0 or new_x >= width:
            break
        while new_pos in board:
            direction = (direction + 1) % 4
            new_pos = (
                pos[0] + DIRECTIONS[direction][0],
                pos[1] + DIRECTIONS[direction][1],
            )
        if is_loop(pos, new_pos, start, direction, board, visited, height, width):
            loops.add(new_pos)
        pos = new_pos

    total = len(loops)
    print_board(board, height, width, loops=loops)
    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
