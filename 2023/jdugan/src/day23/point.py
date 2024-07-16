class Point:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, x, y, value):
        self.id    = (y, x)
        self.x     = x
        self.y     = y
        self.value = value


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def adjacent_ids(self):
        return [
            (self.east_id(),  "E"),
            (self.north_id(), "N"),
            (self.south_id(), "S"),
            (self.west_id(),  "W")
        ]


    # ========== COORD HELPERS ============================

    def east_id(self):
        return (self.y, self.x + 1)

    def north_id(self):
        return (self.y - 1, self.x)

    def south_id(self):
        return (self.y + 1, self.x)

    def west_id(self):
        return (self.y, self.x - 1)