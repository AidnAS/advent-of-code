from src.day02.range import Range
from src.utility.reader import Reader

class Day02:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 2

    def puzzle1(self):
        ranges    = self.__ranges()
        chunks    = self.__chunks_map(ranges, 2)
        checksums = [r.calculate_checksum(chunks) for r in ranges]
        return sum(checksums)

    # 25912654282
    def puzzle2(self):
        ranges = self.__ranges()
        chunks = self.__chunks_map(ranges, 100)
        checksums = [r.calculate_checksum(chunks) for r in ranges]
        return sum(checksums)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __chunks_map(self, ranges, max_chunks):
        chunks = {}
        for r in ranges:
            min_size = len(str(r.start))
            max_size = len(str(r.end))
            for nlen in range(min_size, max_size + 1):
                if nlen not in chunks:
                    chunk_sizes = []
                    chunk_limit = min(max_chunks, nlen)
                    for chunk_size in range(2, chunk_limit + 1):
                        if nlen % chunk_size == 0:
                            chunks_count = nlen // chunk_size
                            chunk_sizes.append(chunks_count)
                    chunks[nlen] = chunk_sizes
        return chunks

    def __ranges(self):
        line = Reader().to_lines("src/day02/data/input.txt")[0]
        strs = line.split(",")
        return [Range(s) for s in strs]