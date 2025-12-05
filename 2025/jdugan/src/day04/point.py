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

    # ========== COORD HELPERS ============================

    def adjacent_ids(self):
        return [
            self.northwest_id(),
            self.north_id(),
            self.northeast_id(),
            self.west_id(),
            self.east_id(),
            self.southwest_id(),
            self.south_id(),
            self.southeast_id(),
        ]

    def east_id(self):
        return (self.y, self.x+1)

    def north_id(self):
        return (self.y-1, self.x)

    def northeast_id(self):
        return (self.y-1, self.x+1)

    def northwest_id(self):
        return (self.y-1, self.x-1)

    def south_id(self):
        return (self.y+1, self.x)

    def southeast_id(self):
        return (self.y+1, self.x+1)

    def southwest_id(self):
        return (self.y+1, self.x-1)

    def west_id(self):
        return (self.y, self.x-1)