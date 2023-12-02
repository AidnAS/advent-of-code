import re
from src.utility.reader import Reader

class Day01:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 1

    def puzzle1(self):
        digits = [self.__calibrate(line) for line in self.__data()]
        return sum(digits)

    def puzzle2(self):
        lines  = [self.__parse(line)     for line in self.__data()]
        digits = [self.__calibrate(line) for line in lines]
        return sum(digits)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calibrate(_, line):
        digits = [int(c) for c in re.sub("[a-z]", "", line)]
        if len(digits) == 1:
            digits.append(digits[0])
        return (10 * digits[0]) + digits[-1]

    def __parse(_, line):
        limit  = len(line)
        digits = []
        pos    = 0
        while pos < limit:
            segment = line[pos:]
            if segment[0] >= "1" and segment[0] <= "9":
                digits.append(line[pos])
            elif segment.startswith("one"):
                digits.append("1")
            elif segment.startswith("two"):
                digits.append("2")
            elif segment.startswith("three"):
                digits.append("3")
            elif segment.startswith("four"):
                digits.append("4")
            elif segment.startswith("five"):
                digits.append("5")
            elif segment.startswith("six"):
                digits.append("6")
            elif segment.startswith("seven"):
                digits.append("7")
            elif segment.startswith("eight"):
                digits.append("8")
            elif segment.startswith("nine"):
                digits.append("9")
            pos += 1
        return "".join(digits)

    def __data(self):
        return Reader().to_lines("data/day01/input.txt")