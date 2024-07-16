class Point:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, x, y, tile, value):
        self.id    = (y, x)
        self.x     = x
        self.y     = y
        self.tile  = tile
        self.value = value


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    # ========== COORD HELPERS ============================

    def adjacent_ids(self):
        return ([
            self.east_id(),
            self.north_id(),
            self.south_id(),
            self.west_id()
        ])

    def east_id(self):
        return (self.y, self.x+1)

    def north_id(self):
        return (self.y-1, self.x)

    def south_id(self):
        return (self.y+1, self.x)

    def west_id(self):
        return (self.y, self.x-1)