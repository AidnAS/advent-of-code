from src.day17.city_map import CityMap
from src.day17.point import Point
from src.utility.reader import Reader

class Day17:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 17

    def puzzle1(self):
        city_map = self.__city_map()
        return city_map.minimum_heat_loss(1, 3)
        # return -1

    def puzzle2(self):
        city_map = self.__city_map()
        return city_map.minimum_heat_loss(4, 10)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day17/input.txt")

    def __city_map(self):
        lines  = self.__data()
        points = {}
        for y, row in enumerate(lines):
            for x, col in enumerate(row):
                points[(y, x)] = Point(x, y, int(col))
        return CityMap(points)