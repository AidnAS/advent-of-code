class Pipe:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, x, y, value):
        self.id      = (y, x)
        self.x       = x
        self.y       = y
        self.value   = value
        self.display = "?"


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def possible_moves(self):
        match self.value:
            case "-":
                return [self.east_id(), self.west_id()]
            case "7":
                return [self.south_id(), self.west_id()]
            case "F":
                return [self.east_id(), self.south_id()]
            case "J":
                return [self.north_id(), self.west_id()]
            case "L":
                return [self.north_id(), self.east_id()]
            case "|":
                return [self.north_id(), self.south_id()]


    # ========== COORD HELPERS ============================

    def adjacent_ids(self):
        return ([
            self.east_id(),
            self.north_id(),
            self.northeast_id(),
            self.northwest_id(),
            self.south_id(),
            self.southeast_id(),
            self.southwest_id(),
            self.west_id()
        ])

    def direction_id(self, dir):
        match dir:
            case "E":
                return self.east_id()
            case "N":
                return self.north_id()
            case "NE":
                return self.northeast_id()
            case "NW":
                return self.northwest_id()
            case "S":
                return self.south_id()
            case "SE":
                return self.southeast_id()
            case "SW":
                return self.southwest_id()
            case "W":
                return self.west_id()

    def east_id(self):
        return (self.y, self.x+1)

    def north_id(self):
        return (self.y+1, self.x)

    def northeast_id(self):
        return (self.y+1, self.x+1)

    def northwest_id(self):
        return (self.y+1, self.x-1)

    def south_id(self):
        return (self.y-1, self.x)

    def southeast_id(self):
        return (self.y-1, self.x+1)

    def southwest_id(self):
        return (self.y-1, self.x-1)

    def west_id(self):
        return (self.y, self.x-1)