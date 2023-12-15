from itertools import combinations
from src.day11.point import Point

class Space:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, points):
        self.points  = points


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def shortest_combined_distance(self):
        ps   = list(combinations(self.points.values(), 2))
        dist = 0
        for (p1, p2) in ps:
            dx = abs(p1.x - p2.x)
            dy = abs(p1.y - p2.y)
            dist += dx + dy
        return dist


    # ========== GRID =====================================

    def expand(self, factor):
        xmap   = self.__expanded_xmap(factor - 1)
        ymap   = self.__expanded_ymap(factor - 1)
        points = {}
        for p in self.points.values():
            x = xmap[p.x] if p.x in xmap else p.x
            y = ymap[p.y] if p.y in ymap else p.y
            points[(y, x)] = Point(x, y, "#")
        return Space(points)

    def print(self):
        dx, dy = self.__dimensions()
        rows = []
        for y in range(0, dy):
            row = ""
            for x in range(0, dx):
                if (y, x) in self.points:
                    row += self.points[(y, x)].value
                else:
                    row += "."
            rows.append(row)
        rows.reverse()
        print("")
        for r in rows:
            print(r)
        print("")


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    # ========== GRID =====================================

    def __dimensions(self):
        x_size = max([p.x for p in self.points.values()]) + 1
        y_size = max([p.y for p in self.points.values()]) + 1
        return (x_size, y_size)

    def __expanded_xmap(self, factor):
        max, _  = self.__dimensions()
        xs      = set([p.x for p in self.points.values()])
        xs      = sorted(list(xs))
        xmap    = self.__expanded_range_map(max, xs, factor)
        return xmap

    def __expanded_ymap(self, factor):
        max, _  = self.__dimensions()
        ys      = set([p.y for p in self.points.values()])
        ys      = sorted(list(ys))
        ymap    = self.__expanded_range_map(max, ys, factor)
        return ymap

    def __expanded_range_map(self, max, ds, factor):
        offsets = {}
        offset  = 0
        for i in range(0, max):
            if i not in ds:
                offset += factor
            offsets[i] = offset
        nds = [d + offsets[d] for i, d in enumerate(ds)]
        hash = {}
        for i, v in enumerate(ds):
            hash[ds[i]] = nds[i]
        return hash