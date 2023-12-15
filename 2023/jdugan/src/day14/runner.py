from src.day14.platform import Platform
from src.day14.point import Point
from src.utility.reader import Reader

class Day14:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 14

    def puzzle1(self):
        platform = self.__platform()
        load     = platform.tilt_once()
        return load

    def puzzle2(self):
        platform = self.__platform()
        load     = platform.spin(1000000000)
        return load


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day14/input.txt")

    def __platform(self):
        points = {}
        for y, row in enumerate(self.__data()):
            for x , col in enumerate(row):
                p = Point(x, y, col)
                points[(y, x)] = p
        return Platform(points)