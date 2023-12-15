from src.day13.pattern import Pattern
from src.day13.point import Point
from src.utility.reader import Reader

class Day13:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 13

    def puzzle1(self):
        patterns = self.__patterns()
        scores   = [p.baseline_score() for p in patterns]
        return sum(scores)

    def puzzle2(self):
        patterns = self.__patterns()
        scores   = [p.smudge_score() for p in patterns]
        return sum(scores)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day13/input.txt")

    def __patterns(self):
        lines    = self.__data()
        patterns = []
        rows     = []
        for line in lines:
            if line == "":
                points = self.__parse_rows(rows)
                patterns.append(Pattern(points))
                rows   = []
            else:
                rows.append(line)
        points = self.__parse_rows(rows)
        patterns.append(Pattern(points))
        return patterns

    def __parse_rows(self, rows):
        points = {}
        for y, row in enumerate(rows):
            for x , col in enumerate(row):
                p = Point(x, y, col)
                points[(y, x)] = p
        return points

