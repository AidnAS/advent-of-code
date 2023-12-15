class Point:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, x, y, value):
        self.id    = (y, x)
        self.x     = x
        self.y     = y
        self.value = value