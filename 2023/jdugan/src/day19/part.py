class Part:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, ratings):
        self.ratings = ratings


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def rating(self):
        vals = [v for k, v in self.ratings.items() if k != "exit"]
        return sum(vals)