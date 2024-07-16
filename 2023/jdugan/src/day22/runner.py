from src.day22.block import Block
from src.day22.tower import Tower
from src.utility.reader import Reader

class Day22:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 22

    def puzzle1(self):
        tower = self.__tower()
        return tower.disintegratable_count()

    def puzzle2(self):
        tower = self.__tower()
        return tower.chain_reaction_count()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day22/input.txt")

    def __tower(self):
        blocks = {}
        for id, line in enumerate(self.__data()):
            coords = []
            parts  = line.split("~")
            for part in parts:
                x, y, z = [int(s) for s in part.split(",")]
                coords.append((x, y, z))
            blocks[id] = Block(id, coords[0], coords[1])
        return Tower(blocks)