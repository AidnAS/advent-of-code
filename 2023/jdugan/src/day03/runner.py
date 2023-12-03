from src.day03.engine   import Engine
from src.utility.reader import Reader

class Day03:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 3

    def puzzle1(self):
        engine = self.__engine()
        parts  = engine.parts()
        ids    = [p.id for p in parts]
        return sum(ids)

    def puzzle2(self):
        engine = self.__engine()
        gears  = engine.gears()
        ratios = [g.ratio() for g in gears]
        return sum(ratios)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(self):
        return Reader().to_lines("data/day03/input.txt")


    def __engine(self):
        lines = self.__data()
        lines.reverse()
        coords = {}
        for y, row in enumerate(lines):
            for x, col in enumerate(row):
                if col != ".":
                    coords[(y,x)] = col
        engine = Engine(coords)
        return engine