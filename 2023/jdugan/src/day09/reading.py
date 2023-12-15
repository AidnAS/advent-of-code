class Reading:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, history):
        self.history = history


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def predict_next(self):
        delta = 0
        for d in self.__calculate_diff_list(self.history):
            delta = d[-1] + delta
        return delta

    def predict_prev(self):
        delta = 0
        for d in self.__calculate_diff_list(self.history):
            delta = d[0] - delta
        return delta


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calculate_diff_list(self, readings):
        diffs = []
        while not (readings[0] == 0 and len(set(readings)) == 1):
            diffs.append(readings)
            readings = self.__calculate_diffs(readings)
        diffs.reverse()
        return diffs

    def __calculate_diffs(self, readings):
        diffs = []
        prev  = readings[0]
        for r in readings[1:]:
            diffs.append(r - prev)
            prev = r
        return diffs