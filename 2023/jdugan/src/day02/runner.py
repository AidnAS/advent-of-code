import re

from src.day02.game     import Game
from src.utility.reader import Reader

class Day02:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 2

    def puzzle1(self):
        ids = [g.id for g in self.__games() if g.within(12, 13, 14)]
        return sum(ids)

    def puzzle2(self):
        powers = [g.calculate_minimum_power() for g in self.__games()]
        return sum(powers)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day02/input.txt")

    def __games(self):
        games = []
        for line in self.__data():
            match         = re.search(r'\AGame (\d+): (.*)\Z', line)
            id, round_str = match.groups()
            game          = Game(int(id), round_str)
            games.append(game)
        return games