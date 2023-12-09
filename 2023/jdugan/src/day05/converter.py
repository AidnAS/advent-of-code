class Converter:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, to_start, from_start, length):
        self.from_start = from_start
        self.to_start   = to_start
        self.length     = length


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def convert_backward(self, num):
        adjust = self.from_start - self.to_start
        if num in range(self.to_start, self.to_start + self.length):
            return num + adjust
        else:
            return num

    def convert_forward(self, num):
        adjust = self.to_start - self.from_start
        if num in range(self.from_start, self.from_start + self.length):
            return num + adjust
        else:
            return num