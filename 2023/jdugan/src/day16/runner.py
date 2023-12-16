from src.day16.contraption import Contraption
from src.day16.point import Point
from src.utility.reader import Reader

class Day16:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 16

    def puzzle1(self):
        contraption = self.__contraption()
        return contraption.origin_energized_count()

    def puzzle2(self):
        contraption = self.__contraption()
        return contraption.maximum_energized_count()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day16/input.txt")

    def __contraption(self):
        lines  = self.__data()
        points = {}
        for y, row in enumerate(lines):
            for x, col in enumerate(row):
                points[(y, x)] = Point(x, y, col)
        return Contraption(points)