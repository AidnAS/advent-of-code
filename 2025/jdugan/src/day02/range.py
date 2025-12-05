from itertools import batched

class Range:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, str):
        parts              = str.split("-")
        self.start         = int(parts[0])
        self.end           = int(parts[1])


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def calculate_checksum(self, chunks):
        ids = []
        for i in range(self.start, self.end + 1):
            nlen = len(str(i))
            for chunk_size in chunks[nlen]:
                if self.__is_invalid(str(i), chunk_size):
                    ids.append(i)
        return sum(set(ids))


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __is_invalid(self, full_str, chunk_size):
        chunks = set(list(batched(list(full_str), chunk_size)))
        return len(chunks) == 1