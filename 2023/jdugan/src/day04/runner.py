import functools
import re
from src.day04.card   import Card
from src.utility.reader import Reader

class Day04:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 4

    def puzzle1(self):
        cards  = self.__cards()
        scores = [c.score() for c in cards]
        return sum(scores)

    def puzzle2(self):
        cards       = self.__cards()
        copy_hash   = {c.id: c.copy_ids() for c in cards}
        copy_counts = {c.id: 1            for c in cards}
        count       = sum(copy_counts.values())
        while len(copy_counts) > 0:
            new_counts  = {}
            for id, factor in copy_counts.items():
                for cid in copy_hash[id]:
                    if cid not in new_counts:
                        new_counts[cid] = 0
                    new_counts[cid] += factor
            copy_counts = new_counts
            count      += sum(copy_counts.values())
        return count


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day04/input.txt")

    def __cards(self):
        lines = self.__data()
        cards = []
        for line in lines:
            outers    = line.split(":")
            inners    = outers[1].split("|")
            id        = int(outers[0].replace("Card ", "").strip())
            winners   = [int(s) for s in inners[0].strip().split()]
            numbers   = [int(s) for s in inners[1].strip().split()]
            card      = Card(id, winners, numbers)
            cards.append(card)
        return cards
