class Landscape:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, pipes):
        self.pipes  = pipes
        self.origin = self.__origin()


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def find_path(self):
        curr     = self.origin
        path     = [curr]
        visited  = { curr.id }
        complete = False
        while not complete:
            possible_ids = [id for id in curr.possible_moves() if id not in visited]
            if len(possible_ids) > 0:
                curr = self.pipes[possible_ids[0]]
                path.append(curr)
                visited.add(curr.id)
            else:
                complete = True
        return path

    def find_interior(self):
        path = self.find_path()
        x      = min([p.x for p in path])
        y      = min([p.y for p in path if p.x == x])
        p0     = self.pipes[(y, x)]
        idx    = path.index(p0)
        path   = path[idx:] + path[0:idx]
        p1     = path[1]
        dir    = "W" if p1.id == p0.north_id() else "S"
        inside = ["N", "NE", "E"]
        for p in path:
            p.display = "."
            match p.value:
                case "7":
                    if dir == "N":
                        dir    = "W"
                        inside = ["W", "SW", "S"] if "W" in inside else ["N", "NE", "E"]
                    else:
                        dir    = "S"
                        inside = ["W", "SW", "S"] if "S" in inside else ["N", "NE", "E"]
                case "F":
                    if dir == "N":
                        dir    = "E"
                        inside = ["S", "SE", "E"] if "E" in inside else ["W", "NW", "N"]
                    else:
                        dir    = "S"
                        inside = ["S", "SE", "E"] if "S" in inside else ["W", "NW", "N"]
                case "J":
                    if dir == "E":
                        dir    = "N"
                        inside = ["W", "NW", "N"] if "N" in inside else ["S", "SE", "E"]
                    else:
                        dir    = "W"
                        inside = ["W", "NW", "N"] if "W" in inside else ["S", "SE", "E"]
                case "L":
                    if dir == "W":
                        dir    = "N"
                        inside = ["N", "NE", "E"] if "N" in inside else ["W", "SW", "S"]
                    else:
                        dir    = "E"
                        inside = ["N", "NE", "E"] if "E" in inside else ["W", "SW", "S"]
            self.__mark_inside(p, inside)
        self.__mark_contiguous_to_insides()
        self.__mark_edges()
        self.__mark_contiguous_to_edges()
        self.__mark_questions()
        # self.print()
        inners = [p for p in self.pipes.values() if p.display == "I"]
        return inners

    def print(self):
        dx, dy = self.__dimensions()
        rows = []
        for y in range(0, dy):
            row = ""
            for x in range(0, dx):
                row += self.pipes[(y, x)].display
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
        x_size = max([p.x for p in self.pipes.values()]) + 1
        y_size = max([p.y for p in self.pipes.values()]) + 1
        return (x_size, y_size)

    def __mark_edges(self):
        dx, dy = self.__dimensions()
        for p in self.pipes.values():
            if p.x in [0, dx-1] or p.y in [0, dy-1]:
                if p.display == "?":
                    p.display = "O"

    def __mark_inside(self, pipe, dirs):
        for dir in dirs:
            id = pipe.direction_id(dir)
            if id in self.pipes:
                if self.pipes[id].display != ".":
                    self.pipes[id].display = "I"

    def __mark_contiguous_to_edges(self):
        outers = [p for p in self.pipes.values() if p.display == "O"]
        while len(outers) > 0:
            new_outers = []
            for op in outers:
                for id in op.adjacent_ids():
                    if id in self.pipes:
                        p = self.pipes[id]
                        if p.display == "?":
                            p.display = "O"
                            new_outers.append(p)
            outers = new_outers

    def __mark_contiguous_to_insides(self):
        inners = [p for p in self.pipes.values() if p.display == "I"]
        while len(inners) > 0:
            new_inners = []
            for ip in inners:
                for id in ip.adjacent_ids():
                    if id in self.pipes:
                        p = self.pipes[id]
                        if p.display == "?":
                            p.display = "I"
                            new_inners.append(p)
            inners = new_inners

    def __mark_questions(self):
        for p in self.pipes.values():
            if p.display == "?":
                p.display = "O"

    def __mark_path(self):
        for p in self.find_path():
            p.display = "."


    # ========== ORIGIN ===================================

    def __origin(self):
        origin = self.__find_origin()
        origin = self.__set_origin_value(origin)
        return origin

    def __find_origin(self):
        origin = None
        for p in self.pipes.values():
            if p.value == "S":
                origin = p
                break
        return origin

    def __set_origin_value(self, origin):
        north = self.pipes.get(origin.north_id())
        south = self.pipes.get(origin.south_id())
        east  = self.pipes.get(origin.east_id())
        west  = self.pipes.get(origin.west_id())
        dirs  = []
        if north and north.value in ["7", "F", "|"]:
            dirs.append("N")
        if south and south.value in ["J", "L", "|"]:
            dirs.append("S")
        if east and east.value in ["-", "7", "J"]:
            dirs.append("E")
        if west and west.value in ["-", "F", "L"]:
            dirs.append("W")
        dirs.sort()
        match dirs:
            case ["E", "W"]:
                origin.value = "-"
            case ["S", "W"]:
                origin.value = "7"
            case ["E", "S"]:
                origin.value = "F"
            case ["N", "W"]:
                origin.value = "J"
            case ["E", "N"]:
                origin.value = "L"
            case ["N", "S"]:
                origin.value = "|"
        return origin