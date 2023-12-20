import re
from src.day18.lagoon import Lagoon
from src.utility.reader import Reader

class Day18:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 18

    def puzzle1(self):
        cmds   = [pair[0] for pair in self.__commands()]
        lagoon = Lagoon(cmds)
        return lagoon.calculate_area()

    def puzzle2(self):
        cmds = [pair[1] for pair in self.__commands()]
        lagoon = Lagoon(cmds)
        return lagoon.calculate_area()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day18/input.txt")

    def __commands(self):
        lines = self.__data()
        cmds  = [self.__parse_line(line) for line in lines]
        return cmds

    def __convert_dir(self, dir):
        match dir:
            case "0":
                return "R"
            case "1":
                return "D"
            case "2":
                return "L"
            case "3":
                return "U"

    def __convert_steps(self, hex_steps):
        return int(hex_steps, 16)

    def __parse_line(self, line):
        result = re.search(r"([DLRU]) (\d+) \W{2}([a-f\d]{5})(\d)\W", line)
        dir1, steps1, steps2, dir2 = result.groups()
        cmd1   = (dir1, int(steps1))
        cmd2   = (self.__convert_dir(dir2), self.__convert_steps(steps2))
        return (cmd1, cmd2)

