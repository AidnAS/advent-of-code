from src.day21.garden import Garden
from src.day21.point import Point
from src.utility.reader import Reader

class Day21:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 21

    def puzzle1(self):
        garden = self.__garden()
        return garden.reachable_points(64)

    def puzzle2(self):
        garden = self.__garden()
        return garden.extremely_reachable_points(26501365)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day21/input.txt")

    def __garden(self):
        points = {}
        tile   = 0
        for y, row in enumerate(self.__data()):
            for x, value in enumerate(row):
                if value != "#":
                    points[(y, x)] = Point(x, y, tile, value)
        return Garden(points)