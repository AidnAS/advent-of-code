import math
from src.day20.modules import Conjunction

class Network:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    INIT_MODULE = "rx"

    def __init__(self, modules):
        self.modules    = modules
        self.high_count = 0
        self.low_count  = 0
        self.history    = {}


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def checksum(self):
        for n in range(1, 1001):
            self.__push_button(n)
        return self.high_count * self.low_count

    def initialization_count(self):
        for n in range(1, 5001):
            self.__push_button(n)
        lcm  = 1
        for f in self.history.values():
            lcm = math.lcm(lcm, f)
        return lcm


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __push_button(self, count):
        pulses = [("button", "broadcaster", "low")]
        while len(pulses) > 0:
            new_pulses = []
            for (source, destination, pulse) in pulses:
                if destination in self.modules:
                    new_pulses += self.modules[destination].receive(source, pulse)
                if pulse == "high":
                    self.high_count += 1
                else:
                    self.low_count += 1
                if source in self.modules:
                    module = self.modules[source]
                    if type(module) is Conjunction:
                        if module.all_high():
                            if source not in self.history:
                                self.history[source] = count
            pulses = new_pulses