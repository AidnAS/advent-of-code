import time
from src.day12.record import Record
from src.utility.reader import Reader

class Day12:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 12

    def puzzle1(self):
        records = self.__records()
        counts  = [r.possible_arrangements() for r in records]
        return sum(counts)

    def puzzle2(self):
        records = self.__expanded_records()
        counts  = [r.possible_arrangements() for r in records]
        return sum(counts)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day12/input.txt")

    def __records(self):
        lines   = self.__data()
        records = []
        for id, line in enumerate(lines):
            springs, pattern = line.split()
            pattern = [int(s) for s in pattern.split(",")]
            record  = Record(id, springs, pattern)
            records.append(record)
        return records

    def __expanded_records(self):
        lines   = self.__data()
        records = []
        for id, line in enumerate(lines):
            base_springs, base_pattern = line.split()
            springs  = []
            patterns = []
            for _ in range(0, 5):
                springs.append(base_springs)
                patterns.append(base_pattern)
            springs = "?".join(springs)
            pattern = ",".join(patterns)
            pattern = [int(s) for s in pattern.split(",")]
            record  = Record(id, springs, pattern)
            records.append(record)
        return records