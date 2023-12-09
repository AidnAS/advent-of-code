import re
from src.day08.node import Node
from src.day08.simulator import Simulator
from src.utility.reader import Reader

class Day08:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 8

    def puzzle1(self):
        simulation = self.__simulation()
        steps      = simulation.fewest_steps(r'AAA', r'ZZZ')
        return steps

    def puzzle2(self):
        simulation = self.__simulation()
        steps      = simulation.fewest_steps(r'\w\wA', r'\w\wZ')
        return steps


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day08/input.txt")

    def __simulation(self):
        lines   = self.__data()
        instrs  = lines[0].strip()
        network = {}
        for line in lines[2:]:
            match       = re.search(r'\A(\w+) = \((\w+), (\w+)\)\Z', line)
            id, l, r    = match.groups()
            node        = Node(id, l, r)
            network[id] = node
        return Simulator(instrs, network)