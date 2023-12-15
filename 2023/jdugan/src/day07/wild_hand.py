class WildHand:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, cards, bid):
        self.bid   = bid
        self.cards = cards


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def sort_key(self):
        key = [self.__type()]
        for c in self.cards:
            match c:
                case "A":
                    key.append(14)
                case "K":
                    key.append(13)
                case "Q":
                    key.append(12)
                case "J":
                    key.append(1)
                case "T":
                    key.append(10)
                case _:
                    key.append(int(c))
        return key


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __apply_wildcards(self, hash):
        jokers = hash.pop("J")
        vals   = sorted(hash.values())
        match vals:
            case [1,1,1,1]:                         # -> 1 pair
                return [1,1,1,2]
            case [1,1,1] | [1,1,2]:                 # -> 3 kind
                return [1,1,3]
            case [2,2]:                             # -> full house
                return [2,3]
            case [1,1] | [1,2] | [1,3]:             # -> 4 kind
                return [1, 4]
            case [] | [1] | [2] | [3] | [4]:        # -> 5 kind
                return [5]

    def __sorted_type_signatures(_):
        return ([
            [1,1,1,1,1],    # high card
            [1,1,1,2],      # 1 pair
            [1,2,2],        # 2 pair
            [1,1,3],        # 3 kind
            [2,3],          # full house
            [1,4],          # 4 kind
            [5]             # 5 kind
        ])

    def __type(self):
        hash = {}
        for c in self.cards:
            if c not in hash:
                hash[c] = 0
            hash[c] += 1
        sigs = self.__sorted_type_signatures()
        if "J" in hash:
            vals = self.__apply_wildcards(hash)
        else:
            vals = sorted(hash.values())
        return sigs.index(vals)