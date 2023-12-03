from src.day03.gear import Gear
from src.day03.part import Part

class Engine:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    PARTS_NEEDED_TO_BE_GEAR = 2

    def __init__(self, coords):
        self.coords = coords


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def gears(self):
        parts = self.parts()
        gears = []
        for sk in self.__star_keys():
            part_ids = [p.id for p in parts if p.adjacent_to(sk)]
            if len(part_ids) == self.PARTS_NEEDED_TO_BE_GEAR:
                gears.append(Gear(part_ids))
        return gears

    def parts(self):
        possibles = self.__possible_parts()
        sym_keys  = self.__symbol_keys()
        parts     = []
        for p in possibles:
            near_symbol = False
            for sk in sym_keys:
                if p.adjacent_to(sk):
                    near_symbol = True
                    break
            if near_symbol:
                parts.append(p)
        return parts


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    #========== PART HELPERS ==============================

    def __possible_parts(self):
        num_keys    = self.__number_keys()
        parts       = []
        curr_id     = ""
        curr_coords = []
        prev_x      = -10
        prev_y      = -10
        for (y, x) in num_keys:
            digit = self.coords[(y, x)]
            if y == prev_y and x == prev_x + 1:
                curr_id += digit
                curr_coords.append((y, x))
            else:
                if len(curr_id) > 0:
                    parts.append(Part(int(curr_id), curr_coords))
                curr_id     = digit
                curr_coords = [(y, x)]
            prev_x = x
            prev_y = y
        parts.append(Part(int(curr_id), curr_coords))
        return parts


    #========== KEY HELPERS ===============================

    def __coord_keys(self):
        keys = list(self.coords.keys())
        keys.sort()
        return keys

    def __number_keys(self):
        all_keys  = self.__coord_keys()
        num_keys  = [k for k in all_keys if self.coords[k] in "0123456789"]
        return num_keys

    def __star_keys(self):
        all_keys  = self.__coord_keys()
        star_keys = [k for k in all_keys if self.coords[k] == "*"]
        return star_keys

    def __symbol_keys(self):
        all_keys  = self.__coord_keys()
        num_keys  = self.__number_keys()
        return [k for k in all_keys if k not in num_keys]