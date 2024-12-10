with open("input.txt") as f:
    lines = f.read().splitlines()

DIRECTIONS = [(-1, 0), (0, 1), (1, 0), (0, -1)]


def part01():
    total = 0
    board = [[int(x) if x != "." else -1 for x in line] for line in lines]

    height, width = len(board), len(board[0])

    queue = []

    for y in range(height):
        for x in range(width):
            if board[y][x] == 0:
                queue.append(((y, x), []))

    while len(queue) > 0:
        next = queue.pop()
        pos, visited = next
        y, x = pos

        value = board[y][x]

        if value == 9:
            total += 1

        for dir in DIRECTIONS:
            ny, nx = y + dir[0], x + dir[1]
            if ny < 0 or ny >= height or nx < 0 or nx >= width:
                continue
            if (ny, nx) in visited:
                continue

            if board[ny][nx] != value + 1:
                continue

            visited.append((ny, nx))
            queue.append(((ny, nx), visited))

    print(f"Part 01: {total}")


def part02():
    total = 0
    board = [[int(x) if x != "." else -1 for x in line] for line in lines]
    height, width = len(board), len(board[0])

    queue = []

    for y in range(height):
        for x in range(width):
            if board[y][x] == 0:
                queue.append(((y, x), []))

    while len(queue) > 0:
        next = queue.pop()
        pos, visited = next
        y, x = pos

        value = board[y][x]

        if value == 9:
            total += 1

        for dir in DIRECTIONS:
            ny, nx = y + dir[0], x + dir[1]
            if ny < 0 or ny >= height or nx < 0 or nx >= width:
                continue
            if (ny, nx) in visited:
                continue

            if board[ny][nx] != value + 1:
                continue

            visited.append((ny, nx))
            queue.append(((ny, nx), visited[::]))

    print(f"Part 02: {total}")


if __name__ == "__main__":
    part01()
    part02()
