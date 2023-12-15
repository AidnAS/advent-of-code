import numpy
from src.utility.reader import Reader

class Day06:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 6

    def puzzle1(self):
        winners = [self.__winner_count(time, dist) for (time, dist) in self.__records()]
        return numpy.prod(winners)

    def puzzle2(self):
        (time, dist) = self.__squashed_record()
        winners      = self.__winner_count(time, dist)
        return winners


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    # ========== RACE =====================================

    def __winner_count(self, time, dist):
        hi = int(time/2)
        lo = 0
        while hi != lo:
            if hi - lo == 1:
                lmax = lo * (time - lo)
                if lmax > dist:
                    hi = lo
                else:
                    lo = hi
            else:
                mid = int((hi + lo)/2)
                mmax = mid * (time - mid)
                if mmax > dist:
                    hi = mid
                else:
                    lo = mid
        return time - (2 * lo) + 1


    # ========== DATA =====================================

    def __data(_):
        return Reader().to_lines("data/day06/input.txt")

    def __records(self):
        lines = self.__data()
        times = [int(s) for s in lines[0].replace("Time:", "").strip().split()]
        dists = [int(s) for s in lines[1].replace("Distance:", "").strip().split()]
        records = []
        for i, t in enumerate(times):
            records.append((t, dists[i]))
        return records

    def __squashed_record(self):
        lines = self.__data()
        time  = int(lines[0].replace("Time:", "").replace(" ", ""))
        dist  = int(lines[1].replace("Distance:", "").replace(" ", ""))
        return (time, dist)
