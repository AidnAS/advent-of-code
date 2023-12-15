from src.day15.initializer import Initializer
from src.utility.reader import Reader

class Day15:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 15

    def puzzle1(self):
        initializer = self.__initializer()
        return initializer.checksum()

    def puzzle2(self):
        initializer = self.__initializer()
        return initializer.focusing_power()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day15/input.txt")

    def __initializer(self):
        line  = self.__data()[0]
        steps = line.split(",")
        return Initializer(steps)
