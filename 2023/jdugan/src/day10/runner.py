import math
from src.day10.landscape import Landscape
from src.day10.pipe import Pipe
from src.utility.reader import Reader

class Day10:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 10

    def puzzle1(self):
        landscape = self.__landscape()
        path      = landscape.find_path()
        return math.ceil(len(path)/2)

    def puzzle2(self):
        landscape = self.__landscape()
        inners    = landscape.find_interior()
        return len(inners)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day10/input.txt")

    def __landscape(self):
        rows = self.__data()
        rows.reverse()
        pipes = {}
        for y, row in enumerate(rows):
            for x, col in enumerate(row):
                p = Pipe(x, y, col)
                pipes[p.id] = p
        return Landscape(pipes)