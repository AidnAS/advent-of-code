class Inventory:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, ranges, ids):
        self.ranges = self.__compact_ranges_recursively(ranges)
        self.ids    = ids


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def count_fresh_ids(self):
        count = 0
        for id in self.ids:
            for r0, r1 in self.ranges:
                if r0 <= id <= r1:
                    count += 1
                    break
        return count

    def total_fresh_ids(self):
        size = 0
        for r0, r1 in self.ranges:
            size += r1 - r0 + 1
        return size


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __compact_ranges_recursively(self, ranges):
        prev = 0
        while prev != len(ranges):
            prev   = len(ranges)
            ranges = self.__compact_ranges(ranges)
        return ranges

    def __compact_ranges(self, ranges):
        ranges.sort()
        compacted = []
        current_start, current_end = ranges[0]
        for r0, r1 in ranges[1:]:
            if r0 <= current_end + 1 and r1 >= current_start - 1:
                current_start = min(current_start, r0)
                current_end   = max(current_end, r1)
            else:
                compacted.append((current_start, current_end))
                current_start, current_end = r0, r1
        compacted.append((current_start, current_end))
        return compacted