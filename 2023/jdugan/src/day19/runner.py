import re
from src.day19.part import Part
from src.day19.sorter import Sorter
from src.utility.reader import Reader

class Day19:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 19

    def puzzle1(self):
        sorter, parts = self.__data()
        ratings       = [p.rating() for p in sorter.accepted_parts(parts)]
        return sum(ratings)

    def puzzle2(self):
        sorter, _ = self.__data()
        return sorter.possible_parts()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        lines = Reader().to_lines("data/day19/input.txt")
        rules = {}
        parts = []
        entry = lines[0].split("{")[0]
        for line in lines:
            if len(line) > 0:
                if line[0] != "{":
                    matches    = re.search(r"(\w+){(.+)}", line)
                    key, cstrs = matches.groups()
                    conditions = []
                    for cstr in cstrs.split(","):
                        if ":" in cstr:
                            matches        = re.search(r"\A(\w+)([=><])(\d+):(\w+)\Z", cstr)
                            a, op, b, dest = matches.groups()
                            cond           = (a, op, int(b), dest)
                            conditions.append(cond)
                        else:
                            cond = ("exit", "=", 1, cstr)
                            conditions.append(cond)
                    rules[key] = conditions
                else:
                    line    = line.replace("{", "").replace("}", "")
                    pairs   = [s.split("=") for s in line.split(",")]
                    ratings = { "exit": 1 }
                    for k, v in pairs:
                        ratings[k] = int(v)
                    parts.append(Part(ratings))
        sorter = Sorter(rules, entry)
        return (sorter, parts)

