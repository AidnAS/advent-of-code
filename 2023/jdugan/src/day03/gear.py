from math import prod

class Gear:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, part_ids):
        self.part_ids = part_ids


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def ratio(self):
        return prod(self.part_ids)