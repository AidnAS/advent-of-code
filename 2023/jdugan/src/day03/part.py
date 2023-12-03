class Part:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, id, coords):
        coords.sort()
        self.id              = id
        self.coords          = coords
        self.adjacent_coords = self.__calculate_adjacent_coords()


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def adjacent_to(self, coord):
        return coord in self.adjacent_coords


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calculate_adjacent_coords(self):
        last_idx = len(self.coords) - 1
        acs      = []
        for idx, (y, x) in enumerate(self.coords):
            if idx == 0:
                acs.append((y-1, x-1))
                acs.append((y, x-1))
                acs.append((y+1, x-1))
            acs.append((y-1, x))
            acs.append((y+1, x))
            if idx == last_idx:
                acs.append((y-1, x+1))
                acs.append((y, x+1))
                acs.append((y+1, x+1))
        return set(acs)
