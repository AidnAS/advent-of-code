class Game:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, id, round_str):
        self.id     = id
        self.rounds = self.__parse_round_str(round_str)


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def calculate_minimum_power(self):
        min = { "r": 0, "g": 0, "b": 0 }
        for r in self.rounds:
            for k in ["r", "g", "b"]:
                if r[k] > min[k]:
                    min[k] = r[k]
        return min["r"] * min["g"] * min["b"]

    def within(self, rlimit, glimit, blimit):
        bounded = True
        for r in self.rounds:
            if r["r"] > rlimit or r["g"] > glimit or r["b"] > blimit:
                bounded = False
                break
        return bounded


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __parse_round_str(_, str):
        rounds     = []
        round_strs = str.split("; ")
        for rs in round_strs:
            rdict  = { "r": 0, "g": 0, "b": 0 }
            groups = rs.split(", ")
            for g in groups:
                parts = g.split(" ")
                rdict[parts[1][0]] = int(parts[0])
            rounds.append(rdict)
        return rounds