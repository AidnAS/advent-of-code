class Hailstone:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, id, x, y, z, dx, dy, dz):
        self.id = id
        self.x  = x
        self.y  = y
        self.z  = z
        self.dx = dx
        self.dy = dy
        self.dz = dz


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def to_vector(self):
        return (self.x, self.y, self.z, self.dx, self.dy, self.dz)

    def xy_formula(self):
        a      = self.dy
        b      = -1 * self.dx
        c      = (self.dx * self.y) - (self.dy * self.x)
        return (a, b, c)

    def xy_intersection(self, other):
        a1, b1, c1 = self.xy_formula()
        a2, b2, c2 = other.xy_formula()
        try:
            x  = (b1*c2 - b2*c1)/(a1*b2 - a2*b1)
            y  = (a2*c1 - a1*c2)/(a1*b2 - a2*b1)
            t1 = (x - self.x)/self.dx
            t2 = (x - other.x)/other.dx
            return ((x, y), t1, t2)
        except:
            return ((None, None), None, None)