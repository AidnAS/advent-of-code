from src.day23.park import Park
from src.day23.point import Point
from src.utility.reader import Reader

class Day23:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 23

    def puzzle1(self):
        park = self.__park()
        return park.longest_anxious_distance()

    def puzzle2(self):
        park = self.__park()
        return park.longest_yolo_distance()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day23/input.txt")

    def __park(self):
        points = {}
        for y, row in enumerate(self.__data()):
            for x, col in enumerate(row):
                if col != "#":
                    p = Point(x, y, col)
                    points[p.id] = p
        return Park(points)