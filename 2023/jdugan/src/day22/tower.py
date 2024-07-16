class Tower:
    # -----------------------------------------------------
    # Configuration
    # -----------------------------------------------------

    def __init__(self, blocks):
        self.blocks = blocks


    # -----------------------------------------------------
    # Public Methods
    # -----------------------------------------------------

    def chain_reaction_count(self):
        counts = {}
        for bid in self.blocks:
            blocks = {}
            for b in self.blocks.values():
                if bid != b.id:
                    blocks[b.id] = b.copy()
            tower  = Tower(blocks)
            ids    = []
            moves  = [bid]
            while len(moves) > 0:
                moves = tower.settle()
                ids  += moves
            ids = set(ids)
            counts[bid] = len(ids)
            print(bid, "=>", len(ids), ids)
        return sum(counts.values())

    def disintegratable_count(self):
        bricks = self.__disintegratable_bricks()
        return len(bricks)


    #========== HACKS =====================================

    # I know this is silly, but I'm pretty sick and
    # this is the best my brain can do right now.
    #
    def rewrite_input(self):
        moves = [0]
        while len(moves) > 0:
            moves = self.settle()
        print("")
        for b in self.blocks.values():
            print(f"{b.x0},{b.y0},{b.z0}~{b.x1},{b.y1},{b.z1}")
        print("")

    def settle(self):
        moves = []
        for b in self.blocks.values():
            on_ground  = b.bottom_points[0][2] == 0
            supporters = []
            for s in self.blocks.values():
                if s.id != b.id:
                    ps = set(b.bottom_points).intersection(set(s.top_points))
                    if len(ps) > 0:
                        supporters.append(s.id)
            if not on_ground and len(supporters) == 0:
                moves.append(b.id)
                b.z0  -= 1
                b.z1  -= 1
                b.recalculate()
        return moves


    # -----------------------------------------------------
    # Private Methods
    # -----------------------------------------------------

    def __calculate_supports(self):
        supports = {}
        for b in self.blocks.values():
            sids = []
            for s in self.blocks.values():
                if s.id != b.id:
                    ps = set(b.top_points).intersection(set(s.bottom_points))
                    if len(ps) > 0:
                        sids.append(s.id)
            supports[b.id] = sids
        return supports

    def __disintegratable_bricks(self):
        supports = self.__calculate_supports()
        bricks   = []
        for bid, sids in supports.items():
            osids = []
            for k, v in supports.items():
                if k != bid:
                    osids += v
            if len(set(sids).intersection(set(osids))) == len(sids):
                bricks.append(self.blocks[bid])
        return bricks