import math

from src.day21.point import Point

class Garden:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, points):
        self.points = points


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def reachable_points(self, steps):
        start_id    = self.__origin().id
        visited_ids = self.__fixed_reachable_points(start_id, steps)
        return len(visited_ids)

    def extremely_reachable_points(self, steps):
        garden       = self.__expand(5)
        tile_counts  = garden.tile_counts(garden.radius())
        tile_radius  = (steps - self.radius()) // self.size() + 1
        center_count = self.__extremely_reachable_center_count(tile_counts, tile_radius)
        offset_count = self.__extremely_reachable_offset_count(tile_counts, tile_radius)
        edge_count   = self.__extremely_reachable_edge_count(tile_counts, tile_radius)
        corner_count = self.__extremely_reachable_corner_count(tile_counts)
        total_count  = center_count + offset_count + edge_count + corner_count
        return total_count


    #========== ATTRIBUTES ================================

    def radius(self):
        return self.size() // 2

    def size(self):
        dy, _ = self.__dimensions()
        return dy

    def tile_counts(self, steps):
        start_id    = self.__origin().id
        visited_ids = self.__fixed_reachable_points(start_id, steps)
        counts      = {}
        for id in visited_ids:
            p = self.points[id]
            if p.tile not in counts:
                counts[p.tile] = 0
            counts[p.tile] += 1
        return counts


    #========== UTILITIES =================================

    def print(self, visited_ids):
        dx, dy = self.__dimensions()
        rows = []
        for y in range(0, dy):
            row = ""
            for x in range(0, dx):
                if (y, x) in visited_ids:
                    row += 'O'
                elif (y, x) in self.points:
                    row += self.points[(y, x)].value
                else:
                    row += '#'
            rows.append(row)
        print("")
        for r in rows:
            print(r)
        print("")


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    #========== GRID HELPERS ==============================

    def __dimensions(self):
        x_size = max([p.x for p in self.points.values()]) + 1
        y_size = max([p.y for p in self.points.values()]) + 1
        return (x_size, y_size)

    def __expand(self, sides):
        dx, dy = self.__dimensions()
        points = {}
        tile   = 0
        for ty in range(0, sides):
            for tx in range(0, sides):
                for p in self.points.values():
                    x = p.x + (tx * dx)
                    y = p.y + (ty * dy)
                    points[(y, x)] = Point(x, y, tile, p.value)
                tile += 1
        center_tile = (sides * sides) // 2
        origins     = [p for p in points.values() if p.value == "S"]
        for p in origins:
            if p.tile != center_tile:
                p.value = '.'
        return Garden(points)

    def __origin(self):
        points = [p for p in self.points.values() if p.value == "S"]
        return points[0]


    #========== COUNT HELPERS =============================

    def __extremely_reachable_center_count(self, tile_counts, tile_radius):
        factor = tile_radius - 2
        return tile_counts[12] * factor * factor

    def __extremely_reachable_corner_count(self, tile_counts):
        north = tile_counts[2]
        west  = tile_counts[10]
        east  = tile_counts[14]
        south = tile_counts[22]
        return north + west + east + south

    def __extremely_reachable_edge_count(self, tile_counts, tile_radius):
        small_factor = tile_radius - 1
        large_factor = tile_radius - 2
        nw_count = (tile_counts[1]  * small_factor) + (tile_counts[6]  * large_factor)
        ne_count = (tile_counts[3]  * small_factor) + (tile_counts[8]  * large_factor)
        se_count = (tile_counts[19] * small_factor) + (tile_counts[18] * large_factor)
        sw_count = (tile_counts[21] * small_factor) + (tile_counts[16] * large_factor)
        return nw_count + ne_count + se_count + sw_count

    def __extremely_reachable_offset_count(self, tile_counts, tile_radius):
        factor = tile_radius - 1
        return tile_counts[13] * factor * factor

    def __fixed_reachable_points(self, start_id, steps):
        visited_ids = set([start_id])
        for step in range(1, steps+1):
            new_points = set()
            for pid in visited_ids:
                point = self.points[pid]
                aids  = [aid for aid in point.adjacent_ids() if aid in self.points]
                for aid in aids:
                    new_points.add(aid)
            visited_ids = new_points
        return visited_ids