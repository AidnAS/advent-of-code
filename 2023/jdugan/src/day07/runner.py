import functools
from src.day07.strict_hand import StrictHand
from src.day07.wild_hand import WildHand
from src.utility.reader import Reader

class Day07:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 7

    def puzzle1(self):
        hands    = self.__sorted_hands(StrictHand)
        winnings = [h.bid * (i+1) for i, h in enumerate(hands)]
        return sum(winnings)

    def puzzle2(self):
        hands    = self.__sorted_hands(WildHand)
        winnings = [h.bid * (i+1) for i, h in enumerate(hands)]
        return sum(winnings)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day07/input.txt")

    def __sorted_hands(self, klass):
        hands = []
        for line in self.__data():
            parts = line.split()
            hand  = klass(parts[0], int(parts[1]))
            hands.append(hand)
        return sorted(hands, key=lambda h: h.sort_key())
