import functools

class Platform:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    MAX_ROTATIONS = 160

    def __init__(self, points):
        self.points = points


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def spin(self, cycles):
        loads = []
        for i in range(0, min([cycles, self.MAX_ROTATIONS])):
            self.__tilt_north()
            self.__tilt_west()
            self.__tilt_south()
            self.__tilt_east()
            loads.append(self.__calculate_load())
        if cycles > self.MAX_ROTATIONS:
            size = self.__find_repeat(loads)
            diff = self.MAX_ROTATIONS - size
            rem = (cycles - diff) % size
            return loads[diff + rem - 1]
        else:
            return loads[cycles - 1]

    def tilt_once(self):
        self.__tilt_north()
        return self.__calculate_load()


    # ========== UTILITY ==================================

    def print(self):
        print("")
        for row in self.__rows():
            print(row)
        print("")


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calculate_load(self):
        _, dy = self.__dimensions()
        load = 0
        for p in self.points.values():
            if p.value == "O":
                load += dy - p.y
        return load

    def __find_repeat(self, loads):
        loads = list(reversed(loads))
        repeat = loads[:2]
        for i in range(2, len(loads)):
            repeat.append(loads[i])
            s0 = i + 1
            s1 = s0 + len(repeat)
            if repeat == loads[s0:s1]:
                break
        return len(repeat)

    def __tilt_east(self):
        dx, dy = self.__dimensions()
        lines  = self.__rows()
        for y, line in enumerate(lines):
            parts = ["".join(sorted([s for s in str])) for str in line.split("#")]
            line  = "#".join(parts)
            for x, s in enumerate(line):
                self.points[(y, x)].value = s
        return self

    def __tilt_north(self):
        dx, dy = self.__dimensions()
        lines  = self.__cols()
        for x, line in enumerate(lines):
            parts = ["".join(sorted([s for s in str])) for str in line.split("#")]
            line  = "#".join(parts)
            line  = "".join(reversed([s for s in line]))
            for y, s in enumerate(line):
                self.points[(y, x)].value = s
        return self

    def __tilt_south(self):
        dx, dy = self.__dimensions()
        lines  = self.__cols()
        for x, line in enumerate(lines):
            parts = ["".join(reversed(sorted([s for s in str]))) for str in line.split("#")]
            line  = "#".join(parts)
            line  = "".join(reversed([s for s in line]))
            for y, s in enumerate(line):
                self.points[(y, x)].value = s
        return self

    def __tilt_west(self):
        dx, dy = self.__dimensions()
        lines  = self.__rows()
        for y, line in enumerate(lines):
            parts = ["".join(reversed(sorted([s for s in str]))) for str in line.split("#")]
            line  = "#".join(parts)
            for x, s in enumerate(line):
                self.points[(y, x)].value = s
        return self


    # ========== GRID =====================================

    def __dimensions(self):
        x_size = max([p.x for p in self.points.values()]) + 1
        y_size = max([p.y for p in self.points.values()]) + 1
        return (x_size, y_size)

    def __cols(self):
        dx, dy = self.__dimensions()
        cols   = []
        for x in range(0, dx):
            col = ""
            for y in range(dy - 1, -1, -1):
                col += self.points[(y, x)].value
            cols.append(col)
        return cols

    def __rows(self):
        dx, dy = self.__dimensions()
        rows   = []
        for y in range(0, dy):
            row = ""
            for x in range(0, dx):
                row += self.points[(y, x)].value
            rows.append(row)
        return rows