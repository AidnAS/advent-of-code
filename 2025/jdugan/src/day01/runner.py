from src.utility.reader import Reader

class Day01:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 1

    def puzzle1(self):
        count, pos, size = 0, 50, 100
        for (dir, dist) in self.__data():
            pos += (dir * dist)
            pos = (pos + size) % size
            if pos == 0:
                count += 1
        return count

    def puzzle2(self):
        count, pos, size = 0, 50, 100
        for (dir, dist) in self.__data():
            count1 = dist // size
            dist1  = dist % size
            pos1   = pos + (dir * dist1)
            if pos != 0 and (pos1 <= 0 or pos1 >= size):
                count1 += 1
            pos    = (pos1 + size) % size
            count += count1
        return count


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(self):
        lines = Reader().to_lines("src/day01/data/input.txt")
        return [self.__parse(line) for line in lines]

    def __parse(self, line):
        dir  = 1 if line[0] == 'R' else -1
        dist = int(line[1:])
        return (dir, dist)
