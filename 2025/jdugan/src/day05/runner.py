from src.day05.inventory import Inventory
from src.utility.reader import Reader

class Day05:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 5

    def puzzle1(self):
        inventory = self.__data()
        return inventory.count_fresh_ids()

    def puzzle2(self):
        inventory = self.__data()
        return inventory.total_fresh_ids()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(self):
        ranges = []
        ids    = []
        lines  = Reader().to_lines("src/day05/data/input.txt")
        for line in lines:
            parts = line.split("-")
            if len(parts) == 2:
                ranges.append((int(parts[0]), int(parts[1])))
            else:
                if parts[0].isdigit():
                    ids.append(int(parts[0]))
        return Inventory(ranges, ids)