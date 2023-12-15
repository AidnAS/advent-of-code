class Record:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, id, springs, pattern):
        self.id      = id
        self.springs = springs
        self.pattern = pattern


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def possible_arrangements(self):
        perms = { (0, 0): 1 }
        for s in self.springs:
            new_perms = {}
            if s in ["#", "?"]:
                for (pidx, slen), copies in perms.items():
                    if pidx < len(self.pattern):
                        if slen < self.pattern[pidx]:
                            key       = (pidx, slen + 1)
                            new_perms = self.__update_perms(new_perms, key, copies)
            if s in [".", "?"]:
                for (pidx, slen), copies in perms.items():
                    if slen == 0:
                        key       = (pidx, slen)
                        new_perms = self.__update_perms(new_perms, key, copies)
                    else:
                        if pidx < len(self.pattern):
                            if slen == self.pattern[pidx]:
                                key       = (pidx + 1, 0)
                                new_perms = self.__update_perms(new_perms, key, copies)
            perms = new_perms
        count = self.__count_matching_perms(perms)
        return count


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __count_matching_perms(self, perms):
        count = 0
        for (pidx, slen), copies in perms.items():
            plen   = len(self.pattern)
            target = self.pattern[plen-1]
            if (pidx, slen) in [(plen-1, target), (plen, 0)]:
                count += copies
        return count

    def __update_perms(self, perms, key, value):
        if key not in perms:
            perms[key] = 0
        perms[key] += value
        return perms