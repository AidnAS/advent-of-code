import copy
from math import prod

class Sorter:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    ENTRY     = "in"
    MAX_VALUE = 4000
    MIN_VALUE = 1

    def __init__(self, rules, entry):
        self.entry = entry
        self.rules = self.__consolidate_rules(rules)


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def accepted_parts(self, parts):
        acceptables = []
        for part in parts:
            name  = self.ENTRY
            found = False
            while not found:
                name = self.__process_rules(self.rules[name], part.ratings)
                match name:
                    case "A":
                        acceptables.append(part)
                        found = True
                    case "R":
                        found = True
        return acceptables

    def possible_parts(self):
        rulesets = self.__combine_rules()
        perms    = [self.__count_permuations(rs) for rs in rulesets]
        return sum(perms)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __combine_rules(self):
        acceptables = []
        possibles   = [(self.ENTRY, { "x": [], "m": [], "a": [], "s": [] })]
        while len(possibles) > 0:
            new_possibles = []
            for (name, ruleset) in possibles:
                ps = self.__unpack_rule(name, ruleset)
                for (name, ruleset) in ps:
                    if name == "A":
                        acceptables.append(ruleset)
                    elif name != "R":
                        new_possibles.append((name, ruleset))
            possibles = new_possibles
        return acceptables

    def __count_permuations(self, ruleset):
        limits = {
            "x": (self.MIN_VALUE, self.MAX_VALUE),
            "m": (self.MIN_VALUE, self.MAX_VALUE),
            "a": (self.MIN_VALUE, self.MAX_VALUE),
            "s": (self.MIN_VALUE, self.MAX_VALUE),
        }
        for k, conditions in ruleset.items():
            minval, maxval = limits[k]
            for (op, val) in conditions:
                match op:
                    case ">":
                        minval = max(minval, val + 1)
                    case ">=":
                        minval = max(minval, val)
                    case "<":
                        maxval = min(maxval, val - 1)
                    case "<=":
                        maxval = min(maxval, val)
            limits[k] = (minval, maxval)
        perms = [max - min + 1 for (min, max) in limits.values()]
        return prod(perms)

    def __invert_condition(self, op, val):
        match op:
            case ">":
                return ("<=", val)
            case "<":
                return (">=", val)
            case _:
                return ("!=", val)

    def __unpack_rule(self, name, ruleset):
        possibles = []
        for (akey, op, bval, dest) in self.rules[name]:
            if akey == "exit":
                rs = copy.deepcopy(ruleset)
                possibles.append((dest, rs))
            else:
                rs = copy.deepcopy(ruleset)
                rs[akey].append((op, bval))
                possibles.append((dest, rs))
                iop, ibval = self.__invert_condition(op, bval)
                ruleset[akey].append((iop, ibval))
        return possibles


    def __consolidate_rules(self, rules):
        always_rejects = []
        for name, conds in rules.items():
            dests = set([dest for (_, _, _, dest) in conds])
            if len(dests) == 1 and "R" in dests:
                always_rejects.append(name)
        consolidated = {}
        for name, conds in rules.items():
            if name not in always_rejects:
                new_conds = []
                for (a, op, b, dest) in conds:
                    if dest in always_rejects:
                        new_conds.append((a, op, b, "R"))
                    else:
                        new_conds.append((a, op, b, dest))
                consolidated[name] = new_conds
        return consolidated

    def __process_rules(self, conditions, ratings):
        name = ""
        for (akey, op, bval, dest) in conditions:
            aval = ratings[akey]
            match op:
                case "=":
                    if aval == bval:
                        name = dest
                        break
                case ">":
                    if aval > bval:
                        name = dest
                        break
                case "<":
                    if aval < bval:
                        name = dest
                        break
        return name