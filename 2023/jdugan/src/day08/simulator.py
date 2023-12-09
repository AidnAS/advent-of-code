import functools
import math
import re

class Simulator:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, instructions, network):
        self.instructions = instructions
        self.network      = network


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def fewest_steps(self, origin_pattern, terminus_pattern):
        origins    = [k for k in self.network if re.match(origin_pattern, k)]
        factors    = []
        for origin in origins:
            (path, steps) = self.__calculate_repeating_path(origin)
            for k in path:
                if re.match(terminus_pattern, k):
                    factors += path[k]
        lcm = functools.reduce(lambda x, y: math.lcm(x, y), factors, 1)
        return lcm


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calculate_repeating_path(self, origin):
        loop_size     = len(self.instructions)
        current       = origin
        uniques       = {(current, 0)}
        path          = {}
        path[current] = [0]
        steps         = 0
        repeated      = False
        while not repeated:
            index   = steps % loop_size
            steps  += 1
            current = self.__move(current, index)
            if not current in path:
                path[current] = []
            path[current].append(steps)
            if (current, steps % loop_size) in uniques:
                repeated = True
            else:
                uniques.add((current, steps % loop_size))
        return (path, steps)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __move(self, current, index):
        if self.instructions[index] == "L":
            return self.network[current].left
        else:
            return self.network[current].right