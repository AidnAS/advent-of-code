from src.day11.point import Point
from src.day11.space import Space
from src.utility.reader import Reader

class Day11:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 11

    def puzzle1(self):
        space = self.__space()
        space = space.expand(2)
        return space.shortest_combined_distance()

    def puzzle2(self):
        space = self.__space()
        space = space.expand(1000000)
        return space.shortest_combined_distance()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day11/input.txt")

    def __space(self):
        rows = self.__data()
        rows.reverse()
        points = {}
        for y, row in enumerate(rows):
            for x, col in enumerate(row):
                if col == "#":
                    p = Point(x, y, col)
                    points[p.id] = p
        return Space(points)
