from itertools import combinations
import z3

class Storm:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, hailstones):
        self.hailstones = hailstones


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def magic_rock(self):
        solver = z3.Solver()
        rx     = z3.Real('rx')
        ry     = z3.Real('ry')
        rz     = z3.Real('rz')
        rdx    = z3.Real('rdx')
        rdy    = z3.Real('rdy')
        rdz    = z3.Real('rdz')
        for idx, hs in enumerate(self.hailstones.values()):
            t = z3.Real(f't{idx}')
            solver.add(t >= 0)
            solver.add(hs.x + hs.dx * t == rx + rdx * t)
            solver.add(hs.y + hs.dy * t == ry + rdy * t)
            solver.add(hs.z + hs.dz * t == rz + rdz * t)
        solver.check()
        model = solver.model()
        x = model.eval(rx).as_long()
        y = model.eval(ry).as_long()
        z = model.eval(rz).as_long()
        return x + y + z

    def sample_size(self, lower, upper):
        count  = 0
        combos = combinations(self.hailstones.values(), 2)
        for h1, h2 in combos:
            (x, y), t1, t2 = h1.xy_intersection(h2)
            if x:
                if (x >= lower and x <= upper) and (y >= lower and y <= upper) and t1 > 0 and t2 > 0:
                    count += 1
        return count