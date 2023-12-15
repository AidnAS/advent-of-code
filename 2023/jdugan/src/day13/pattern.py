class Pattern:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    MISSING_REFLECTION = (" ", 0)

    def __init__(self, points):
        self.points = points


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def baseline_score(self):
        t, v = self.__find_reflection()
        return self.__calculate_score(t, v)

    def smudge_score(self):
        score    = 0
        baseline = self.__find_reflection()
        for p in self.points.values():
            p.invert()
            (t, v) = self.__find_reflection(baseline)
            if (t, v) != self.MISSING_REFLECTION:
                score = self.__calculate_score(t, v)
                break
            else:
                p.invert()
        return score


    # ========== UTILITY ==================================

    def print(self):
        print("")
        for row in self.__rows():
            print(row)
        print("")


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calculate_score(self, type, value):
        match type:
            case "H":
                return value * 100
            case _:
                return value

    def __find_reflection(self, ignore=MISSING_REFLECTION):
        value = self.__find_reflection_for_orientation(self.__rows(), "H", ignore)
        if value > 0:
            return ("H", value)
        else:
            value = self.__find_reflection_for_orientation(self.__cols(), "V", ignore)
            if value > 0:
                return ("V", value)
            else:
                return self.MISSING_REFLECTION

    def __find_reflection_for_orientation(self, lines, type, ignore):
        value = 0
        for index in range(0, len(lines) - 1):
            ids0  = [n for n in range(index, -1, -1)]
            ids1  = [n for n in range(index + 1, len(lines))]
            clen  = min([len(ids0), len(ids1)])
            found = all([lines[ids0[i]] == lines[ids1[i]] for i in range(0, clen)])
            if found:
                if (type, ids1[0]) != ignore:
                    value = ids1[0]
                    break
        return value


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