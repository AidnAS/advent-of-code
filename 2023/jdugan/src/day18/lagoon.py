class Lagoon:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, commands):
        self.commands = commands
        self.vertices = self.__build_vertices(commands)


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def calculate_area(self):
        shoelace  = self.__calculate_shoelace()
        perimeter = self.__calculate_perimeter()
        return shoelace + int(perimeter/2) + 1


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calculate_shoelace(self):
        length = len(self.vertices)
        sum1   = 0
        sum2   = 0
        for idx0 in range(0, length):
            idx1  = (idx0 + 1) % length
            v0    = self.vertices[idx0]
            v1    = self.vertices[idx1]
            sum1 += v0[0] * v1[1]
            sum2 += v0[1] * v1[0]
        return int(abs(sum1 - sum2)/2)

    def __calculate_perimeter(self):
        lengths = [pair[1] for pair in self.commands]
        return sum(lengths)


    # ========== VERTICES =================================

    def __build_vertices(self, commands):
        x, y     = (0, 0)
        vertices = []
        for (dir, steps) in commands:
            dx, dy = self.__movement_factors(dir)
            x += steps * dx
            y += steps * dy
            vertices.append((x, y))
        if self.__vertices_inverted(commands[0], commands[1]):
            vertices.reverse()
        return vertices

    def __movement_factors(self, dir):
        match dir:
            case "D":
                return (0, 1)
            case "L":
                return (-1, 0)
            case "R":
                return (1, 0)
            case "U":
                return (0, -1)

    def __vertices_inverted(self, cmd1, cmd2):
        match (cmd1[0], cmd2[0]):
            case ("D", "R") | ("L", "D") | ("R", "U") | ("U", "L"):
                return True
            case _:
                return False