from dijkstar import Graph, find_path

class Park:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, points):
        self.points = points


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def longest_anxious_distance(self):
        valid_fn = self.__anxious_valid_fn
        return self.__longest_distance(valid_fn)

    def longest_yolo_distance(self):
        valid_fn = self.__confident_valid_fn
        return self.__longest_distance(valid_fn)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __longest_distance(self, valid_fn):
        origin    = self.__origin_id()
        terminus  = self.__terminus_id()
        network   = self.__build_network(valid_fn)
        paths     = self.__calculate_paths(network, origin, terminus)
        distances = self.__calculate_distances(network, paths)
        return max(distances)


    #========== CALCULATION HELPERS =======================

    def __build_network(self, valid_fn):
        junctions = self.__find_junctions()
        network   = {}
        for origin in junctions:
            network[origin] = {}
            paths           = []
            for (aid, dir) in self.points[origin].adjacent_ids():
                if valid_fn(aid, dir):
                    paths.append([origin, aid])
            for p in paths:
                dest  = p[-1]
                valid = True
                while valid and dest not in junctions:
                    pairs    = [(aid, dir) for (aid, dir) in self.points[dest].adjacent_ids() if aid in self.points and aid not in p]
                    aid, dir = pairs[0]
                    if valid_fn(aid, dir):
                        p.append(aid)
                        dest = aid
                    else:
                        valid = False
                if valid:
                    network[origin][dest] = len(p) - 1
        return network

    def __calculate_distances(self, network, paths):
        distances = []
        for path in paths:
            d = 0
            origin = path[0]
            for terminus in path[1:]:
                d += network[origin][terminus]
                origin = terminus
            distances.append(d)
        return distances

    def __calculate_paths(self, network, origin, terminus):
        paths     = []
        possibles = [[origin]]
        while len(possibles) > 0:
            new_possibles = []
            for p in possibles:
                curr_id  = p[-1]
                next_ids = [id for id in network[curr_id] if id not in p]
                for next_id in next_ids:
                    np = p + [next_id]
                    if next_id == terminus:
                        paths.append(np)
                    else:
                        new_possibles.append(np)
            possibles = new_possibles
        return paths

    def __find_junctions(self):
        junctions = []
        for p in self.points.values():
            aids = []
            for (aid, _) in p.adjacent_ids():
                if aid in self.points:
                    aids.append(aid)
            if len(aids) != 2:
                junctions.append(p.id)
        return junctions


    #========== VALIDITY HELPERS ==========================

    def __anxious_valid_fn(self, id, dir):
        valids = {
            "E": { ".", ">" },
            "N": { ".", "^" },
            "S": { ".", "v" },
            "W": { ".", "<" }
        }
        return id in self.points and self.points[id].value in valids[dir]

    def __confident_valid_fn(self, id, _):
        return id in self.points


    #========== GRID HELPERS ==============================

    def __dimensions(self):
        x_size = max([p.x for p in self.points.values()]) + 1
        y_size = max([p.y for p in self.points.values()]) + 1
        return (x_size, y_size)

    def __origin_id(self):
        x = 0
        y = 0
        for p in self.points.values():
            if p.y == y:
                x = p.x
                break
        return (y, x)

    def __terminus_id(self):
        _, dy = self.__dimensions()
        x = 0
        y = dy - 1
        for p in self.points.values():
            if p.y == y:
                x = p.x
                break
        return (y, x)