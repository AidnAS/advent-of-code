from igraph import Graph
from math import prod

class Network:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, connections):
        self.connections = connections


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def repaired_size(self):
        graph  = Graph.ListDict(self.connections)
        groups = graph.mincut()
        sizes  = groups.sizes()
        return prod(sizes)