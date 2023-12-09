import re
from src.day05.almanac import Almanac
from src.day05.converter import Converter
from src.utility.reader import Reader

class Day05:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 5

    def puzzle1(self):
        almanac = self.__almanac()
        return almanac.lowest_from_list()

    def puzzle2(self):
        almanac = self.__almanac()
        return almanac.lowest_from_ranges()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day05/input.txt")

    def __almanac(self):
        lines = self.__data()
        line  = lines[0]
        seeds = [int(s) for s in line.replace("seeds: ", "").split()]

        converters      = {}
        type            = None
        type_converters = []
        for line in lines[2:]:
            match = re.search(r'\A(\w+)-to-(\w+) map:\Z', line)
            if match:
                _, type         = match.groups()
            elif len(line) > 0:
                parts = [int(s) for s in line.split()]
                c = Converter(parts[0], parts[1], parts[2])
                type_converters.append(c)
            else:
                converters[type] = type_converters
                type_converters  = []
        converters[type] = type_converters

        return Almanac(seeds, converters)