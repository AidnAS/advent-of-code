from string import digits
from src.utility.reader import Reader

class Day03:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 3

    def puzzle1(self):
        sum = 0
        for line in self.__data():
            sum += self.__calculate_joltage(line, 2)
        return sum

    def puzzle2(self):
        sum = 0
        for line in self.__data():
            sum += self.__calculate_joltage(line, 12)
        return sum


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calculate_joltage(self, line, size):
        digits = self.__find_largest(list(), line, size)
        strs = [str(i) for i in digits]
        s    = ''.join(strs)
        return int(s)

    def __find_largest(self, digits, line, size):
        if size == 0:
            return digits
        else:
            size -= 1
            search_str = line[:len(line) - size]
            remain_str = line[len(line) - size:]
            max_val = 0
            max_idx = -1
            for idx, val in enumerate(search_str):
                if int(val) > max_val:
                    max_val = int(val)
                    max_idx = idx
            digits.append(max_val)
            line = search_str[max_idx + 1:] + remain_str
            return self.__find_largest(digits, line, size)

    def __data(self):
        return Reader().to_lines("src/day03/data/input.txt")
