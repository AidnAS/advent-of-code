from src.day24.hailstone import Hailstone
from src.day24.storm import Storm
from src.utility.reader import Reader

class Day24:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 24

    def puzzle1(self):
        storm = self.__storm()
        return storm.sample_size(200000000000000, 400000000000000)

    def puzzle2(self):
        storm = self.__storm()
        return storm.magic_rock()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day24/input.txt")

    def __storm(self):
        hailstones = {}
        for id, line in enumerate(self.__data()):
            coords, deltas = line.split(" @ ")
            x, y, z        = coords.split(", ")
            dx, dy, dz     = deltas.split(", ")
            hailstone      = Hailstone(id, int(x), int(y), int(z), int(dx), int(dy), int(dz))
            hailstones[id] = hailstone
        return Storm(hailstones)