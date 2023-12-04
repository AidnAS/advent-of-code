import math

class Card:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, id, winners, numbers):
        self.id   = id
        self.wins = len([p for p in numbers if p in winners])


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def score(self):
        return int(math.pow(2, self.wins-1)) if self.wins > 0 else 0

    def copy_ids(self):
        return [self.id+i for i in range(1, self.wins+1)]