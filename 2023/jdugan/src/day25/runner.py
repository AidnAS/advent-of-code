from src.day25.network import Network
from src.utility.reader import Reader

class Day25:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 25

    def puzzle1(self):
        network = self.__network()
        return network.repaired_size()

    def puzzle2(self):
        return -2


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day25/input.txt")

    def __network(self):
        connections = {}
        for line in self.__data():
            parts   = line.split(": ")
            origin  = parts[0]
            connections[origin] = list(parts[1].split(" "))
        return Network(connections)