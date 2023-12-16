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

    def next_steps(self, dir):
        vals = []
        match (self.value, dir):
            case (".", "E") | ("-", "E") | ("\\", "S") | ("/", "N"):
                v1 = (self.east_id(), "E")
                vals.append(v1)
            case (".", "N") | ("|", "N") | ("\\", "W") | ("/", "E"):
                v1 = (self.north_id(), "N")
                vals.append(v1)
            case (".", "S") | ("|", "S") | ("\\", "E") | ("/", "W"):
                v1 = (self.south_id(), "S")
                vals.append(v1)
            case (".", "W") | ("-", "W") | ("\\", "N") | ("/", "S"):
                v1 = (self.west_id(), "W")
                vals.append(v1)
            case ("-", "N") | ("-", "S"):
                v1 = (self.east_id(), "E")
                v2 = (self.west_id(), "W")
                vals += [v1, v2]
            case ("|", "E") | ("|", "W"):
                v1 = (self.north_id(), "N")
                v2 = (self.south_id(), "S")
                vals += [v1, v2]
        return vals


    # ========== COORD HELPERS ============================

    def east_id(self):
        return (self.y, self.x+1)

    def north_id(self):
        return (self.y-1, self.x)

    def south_id(self):
        return (self.y+1, self.x)

    def west_id(self):
        return (self.y, self.x-1)