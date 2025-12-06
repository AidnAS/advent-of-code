import re

from src.day06.equation import Equation
from src.utility.reader import Reader

class Day06:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 6

    def puzzle1(self):
        eqs   = self.__human_equations()
        calcs = [eq.calculate() for eq in eqs]
        return sum(calcs)

    def puzzle2(self):
        eqs   = self.__cephalopod_equations()
        calcs = [eq.calculate() for eq in eqs]
        return sum(calcs)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(self):
        return Reader().to_raw_lines("src/day06/data/input.txt")

    def __cephalopod_equations(self):
        eqs   = []
        lines = self.__data()
        cols  = {}
        for line in lines:
            for i, s in enumerate(line):
                if i not in cols:
                    cols[i] = []
                cols[i].append(s)
        operator = ""
        terms    = []
        for i in range(len(cols)):
            term = "".join(cols[i]).strip()
            if term == "":
                eq = Equation(operator, terms)
                eqs.append(eq)
                operator = ""
                terms    = []
            else:
                if term[-1] in ["+", "*"]:
                    operator = term[-1]
                    terms.append(int(term[:-1]))
                else:
                    terms.append(int(term))
        return eqs

    def __human_equations(self):
        eqs   = []
        lines = self.__data()
        lines.reverse()
        for i, line in enumerate(lines):
            parts = re.split(r"\s+", line.strip())
            for j, part in enumerate(parts):
                if i == 0:
                    eqs.append(Equation(part, []))
                else:
                    eqs[j].add_term(int(part))
        return eqs