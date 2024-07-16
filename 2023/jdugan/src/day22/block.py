class Block:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, id, start_coord, end_coord):
        x0, y0, z0 = start_coord
        x1, y1, z1 = end_coord

        self.id = id
        self.x0 = x0
        self.y0 = y0
        self.z0 = z0
        self.x1 = x1
        self.y1 = y1
        self.z1 = z1

        self.bottom_points = []
        self.top_points    = []
        self.recalculate()


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def copy(self):
        id = self.id
        c0 = (self.x0, self.y0, self.z0)
        c1 = (self.x1, self.y1, self.z1)
        return Block(id, c0, c1)

    def recalculate(self):
        self.__calculate_bottom_points()
        self.__calculate_top_points()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    # ========== COORD HELPERS ============================

    def __calculate_bottom_points(self):
        z      = self.z0 - 1
        coords = []
        for x in range(self.x0, self.x1 + 1):
            for y in range(self.y0, self.y1 + 1):
                coords.append((x, y, z))
        self.bottom_points = coords
        self.bottom_points.sort()

    def __calculate_top_points(self):
        z      = self.z1
        coords = []
        for x in range(self.x0, self.x1 + 1):
            for y in range(self.y0, self.y1 + 1):
                coords.append((x, y, z))
        self.top_points = coords
        self.top_points.sort()