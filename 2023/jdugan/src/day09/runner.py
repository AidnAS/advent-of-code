from src.day09.reading import Reading
from src.utility.reader import Reader

class Day09:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 9

    def puzzle1(self):
        readings    = self.__readings()
        predictions = [r.predict_next() for r in readings]
        return sum(predictions)

    def puzzle2(self):
        readings    = self.__readings()
        predictions = [r.predict_prev() for r in readings]
        return sum(predictions)


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day09/input.txt")

    def __readings(self):
        lines    = self.__data()
        readings = []
        for line in lines:
            history = [int(s) for s in line.split()]
            reading = Reading(history)
            readings.append(reading)
        return readings
