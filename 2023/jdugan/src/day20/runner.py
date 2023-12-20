from src.day20.modules import Broadcaster, Conjunction, FlipFlop
from src.day20.network import Network
from src.utility.reader import Reader

class Day20:
    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def day(_):
        return 20

    def puzzle1(self):
        network = self.__network()
        return network.checksum()

    def puzzle2(self):
        network = self.__network()
        return network.initialization_count()


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __data(_):
        return Reader().to_lines("data/day20/input.txt")

    def __network(self):
        conjs   = []
        modules = {}
        for line in self.__data():
            nstr, dstr    = line.split(" -> ")
            name, klass   = self.__parse_name(nstr)
            dests         = dstr.split(", ")
            if klass == Conjunction:
                conjs.append(name)
            module        = klass(name, dests)
            modules[name] = module
        for c in conjs:
            sources = []
            for m in modules.values():
                if c in m.destinations:
                    sources.append(m.name)
            modules[c].initialize_memory(sources)
        return Network(modules)

    def __parse_name(self, name_str):
        if name_str == "broadcaster":
            return ("broadcaster", Broadcaster)
        elif name_str[0] == "&":
            return (name_str[1:], Conjunction)
        else:
            return (name_str[1:], FlipFlop)