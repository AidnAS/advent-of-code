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

    def adjacent_ids(self, distance):
        return [
            (self.east_id(distance),  "E"),
            (self.north_id(distance), "N"),
            (self.south_id(distance), "S"),
            (self.west_id(distance),  "W")
        ]

    def invert_direction(self, dir):
        match dir:
            case "E":
                return "W"
            case "N":
                return "S"
            case "S":
                return "N"
            case "W":
                return "E"


    # ========== COORD HELPERS ============================

    def east_id(self, distance):
        return (self.y, self.x + distance)

    def north_id(self, distance):
        return (self.y - distance, self.x)

    def south_id(self, distance):
        return (self.y + distance, self.x)

    def west_id(self, distance):
        return (self.y, self.x - distance)