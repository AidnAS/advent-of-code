class Initializer:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, steps):
        self.boxes = self.__boxes(steps)
        self.steps = steps


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def checksum(self):
        vals = [self.__hash(s) for s in self.steps]
        return sum(vals)

    def focusing_power(self):
        power = 0
        for id, lenses in self.boxes.items():
            f1 = id + 1
            for slot, (_, flen) in enumerate(lenses):
                f2 = slot + 1
                f3 = flen
                power += (f1 * f2 * f3)
        return power


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __boxes(self, steps):
        boxes = {}
        for i in range(0, 256):
            boxes[i] = []
        for step in steps:
            if "=" in step:
                self.__process_equal(boxes, step)
            else:
                self.__process_minus(boxes, step)
        return boxes

    def __hash(self, step):
        val = 0
        for s in step:
            val += ord(s)
            val *= 17
            val  = val % 256
        return val

    def __process_equal(self, boxes, step):
        label, flen = step.split("=")
        id          = self.__hash(label)
        flen        = int(flen)
        found       = False
        for i, (l, fl) in enumerate(boxes[id]):
            if l == label:
                boxes[id][i] = (label, flen)
                found = True
        if not found:
            boxes[id].append((label, flen))

    def __process_minus(self, boxes, step):
        label     = step[:-1]
        id        = self.__hash(label)
        boxes[id] = [t for t in boxes[id] if t[0] != label]
        return boxes