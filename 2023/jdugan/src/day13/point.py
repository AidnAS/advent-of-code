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
    # Public Method
    # -----------------------------------------------------

    def invert(self):
        if self.value == "#":
            self.value = "."
        else:
            self.value = "#"