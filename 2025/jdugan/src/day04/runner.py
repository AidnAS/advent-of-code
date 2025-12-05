from src.day04.point import Point
from src.utility.reader import Reader

class Day04:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 4

    def puzzle1(self):
        return self.__remove_rolls_once(self.__grid())

    def puzzle2(self):
        return self.__remove_rolls_recursively(self.__grid(), 0)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __remove_rolls_once(self, grid):
        _, count = self.__remove_rolls(grid, 0)
        return count

    def __remove_rolls_recursively(self, grid, count):
        new_grid, new_count = self.__remove_rolls(grid, count)
        if new_count > count:
            return self.__remove_rolls_recursively(new_grid, new_count)
        return new_count

    def __remove_rolls(self, grid, total_count):
        count = 0
        ids   = []
        for id, point in grid.items():
            num = 0
            for adjacent_id in point.adjacent_ids():
                if adjacent_id in grid:
                    num += 1
            if num < 4:
                ids.append(id)
                count += 1
        total_count += count
        for id in ids:
            del grid[id]
        return (grid, total_count)

    def __grid(self):
        grid  = {}
        lines = Reader().to_lines("src/day04/data/input.txt")
        for y, line in enumerate(lines):
            for x, char in enumerate(line):
                if char == "@":
                    grid[(y, x)] = Point(x, y, char)
        return grid
