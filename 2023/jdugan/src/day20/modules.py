#========== ABSTRACT CLASSES ==============================

class AbstractModule:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, name, destinations):
        self.name         = name
        self.destinations = destinations


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def receive(self, _, pulse):
        pass

    def send(self, pulse):
        return [(self.name, d, pulse) for d in self.destinations]


#========== CONCRETE CLASSES ==============================

class Broadcaster(AbstractModule):
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def receive(self, _, pulse):
        return self.send(pulse)


class Conjunction(AbstractModule):
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, name, destinations):
        super().__init__(name, destinations)
        self.memory = {}


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def all_high(self):
        pulses = set([v for v in self.memory.values()])
        return len(pulses) == 1 and "high" in pulses

    def initialize_memory(self, sources):
        for s in sources:
            self.memory[s] = "low"

    def receive(self, source, pulse):
        self.memory[source] = pulse
        if self.all_high():
            return self.send("low")
        else:
            return self.send("high")


class FlipFlop(AbstractModule):
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, name, destinations):
        super().__init__(name, destinations)
        self.on = False


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def receive(self, _, pulse):
        if pulse == "low":
            if self.on:
                self.on = False
                return self.send("low")
            else:
                self.on = True
                return self.send("high")
        else:
            return []