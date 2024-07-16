from dijkstar import Graph, find_path
from src.day16.point import Point

class CityMap:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    MAX_STEPS = 3

    def __init__(self, points):
        self.points     = points
        self.dimensions = self.__dimensions()


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def minimum_heat_loss(self, min_steps, max_steps):
        dx, dy    = self.dimensions
        graph     = self.__graph(min_steps, max_steps)
        distances = []
        origins   = [(0, 0, "N", min_steps), (0, 0, "W", min_steps)]
        termini   = []
        for dir in ["E", "S"]:
            for step in range(min_steps, max_steps + 1):
                termini.append((dy-1, dx-1, dir, step))
        for origin in origins:
            for terminus in termini:
                p = find_path(graph, origin, terminus)
                distances.append(p.total_cost)
        return min(distances)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __dimensions(self):
        x_size = max([p.x for p in self.points.values()]) + 1
        y_size = max([p.y for p in self.points.values()]) + 1
        return (x_size, y_size)

    def __graph(self, min_steps, max_steps):
        dx, dy = self.dimensions
        graph  = Graph()
        for y in range(0, dy):
            for x in range(0, dx):
                for i in range(1, max_steps + 1):
                    fpoint    = self.points[(y, x)]
                    required  = max([1, min_steps - i])
                    to_dirs   = [dir for (id, dir) in fpoint.adjacent_ids(required) if id in self.points]
                    to_points = [(id, dir) for (id, dir) in fpoint.adjacent_ids(1) if dir in to_dirs]
                    from_dirs = [fpoint.invert_direction(dir) for (id, dir) in fpoint.adjacent_ids(i) if id in self.points]
                    for fdir in from_dirs:
                        from_node = (fpoint.y, fpoint.x, fdir, i)
                        for (tid, tdir) in to_points:
                            tpoint = self.points[tid]
                            if tdir == fdir:
                                if i < max_steps:
                                    to_node = (tpoint.y, tpoint.x, tdir, i + 1)
                                    graph.add_edge(from_node, to_node, tpoint.value)
                            elif tdir != fpoint.invert_direction(fdir):
                                if i >= min_steps:
                                    to_node = (tpoint.y, tpoint.x, tdir, 1)
                                    graph.add_edge(from_node, to_node, tpoint.value)
        return graph