from src.day05.converter import Converter

class Almanac:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    SEARCH_FLOOR = 11554000

    def __init__(self, seeds, converters):
        self.seeds      = seeds
        self.converters = converters


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def lowest_from_list(self):
        locs = [self.__calculate_location(s) for s in self.seeds]
        return min(locs)

    def lowest_from_ranges(self):
        chunks = [self.seeds[i:i + 2] for i in range(0, len(self.seeds), 2)]
        ranges = [range(c[0], c[0] + c[1]) for c in chunks]
        r0     = self.SEARCH_FLOOR
        r1     = self.SEARCH_FLOOR + 1000000
        lowest = -2
        for i in range(r0, r1):
            seed = self.__calculate_seed(i)
            if any([seed in r for r in ranges]):
                lowest = i
                break
        return lowest


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calculate_location(self, seed):
        n1 = self.__process_forward("soil", seed)
        n2 = self.__process_forward("fertilizer", n1)
        n3 = self.__process_forward("water", n2)
        n4 = self.__process_forward("light", n3)
        n5 = self.__process_forward("temperature", n4)
        n6 = self.__process_forward("humidity", n5)
        n7 = self.__process_forward("location", n6)
        return n7

    def __calculate_seed(self, loc):
        n1 = self.__process_backward("location", loc)
        n2 = self.__process_backward("humidity", n1)
        n3 = self.__process_backward("temperature", n2)
        n4 = self.__process_backward("light", n3)
        n5 = self.__process_backward("water", n4)
        n6 = self.__process_backward("fertilizer", n5)
        n7 = self.__process_backward("soil", n6)
        return n7

    def __process_backward(self, type, num):
        for c in self.converters[type]:
            n = c.convert_backward(num)
            if n != num:
                return n
        return num

    def __process_forward(self, type, num):
        for c in self.converters[type]:
            n = c.convert_forward(num)
            if n != num:
                return n
        return num