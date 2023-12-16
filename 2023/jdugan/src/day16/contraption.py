from src.day16.point import Point

class Contraption:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, points):
        self.points     = points
        self.dimensions = self.__dimensions()


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def maximum_energized_count(self):
        dx, dy = self.dimensions
        beams  = []
        beams += [(self.points[(y, 0)],    "E") for y in range(0, dy)]
        beams += [(self.points[(y, dx-1)], "W") for y in range(0, dy)]
        beams += [(self.points[(0, x)],    "S") for x in range(0, dx)]
        beams += [(self.points[(dy-1, x)], "N") for x in range(0, dx)]
        counts = [self.__energized_count(beam) for beam in beams]
        return max(counts)

    def origin_energized_count(self):
        beam = (self.points[(0, 0)], "E")
        return self.__energized_count(beam)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __dimensions(self):
        x_size = max([p.x for p in self.points.values()]) + 1
        y_size = max([p.y for p in self.points.values()]) + 1
        return (x_size, y_size)

    def __energized_count(self, beam):
        working_beams = [beam]
        visited       = set()
        visited.add((beam[0].id, beam[1]))
        while len(working_beams):
            new_beams = []
            for (point, dir) in working_beams:
                steps = point.next_steps(dir)
                for (nid, ndir) in steps:
                    if nid in self.points and (nid, ndir) not in visited:
                        visited.add((nid, ndir))
                        new_beams.append((self.points[nid], ndir))
            working_beams = new_beams
        ids = set()
        for (id, _) in visited:
            ids.add(id)
        return len(ids)