package day20

import (
	"aoc/2024/pkg/utility"
	"fmt"

	"github.com/RyanCarrier/dijkstra/v2"
	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Maze struct {
	points        map[string]Point
	best_distance int
	best_path     []string
}

// ========== RECEIVERS ===================================

func (m Maze) ShortcutCount(cheats int, saving int) int {
	shortcuts := m.FindShortcuts(cheats, saving)
	return len(shortcuts)
}

func (m Maze) FindShortcuts(cheat int, saving int) []string {
	shortcuts := make([]string, 0)
	head, tail := pie.Shift(m.best_path)
	hidx := 0
	for len(head) > 0 {
		hp := m.points[head]
		for tidx, tid := range tail {
			tp := m.points[tid]
			dist := hp.DistanceTo(tp)
			if dist >= 2 && dist <= cheat {
				total := hidx + dist + len(tail) - tidx - 1
				delta := m.best_distance - total
				if delta >= saving {
					// sc := Shortcut{p1: hp, p2: tp, delta: delta}
					sc := hp.Id() + ";" + tp.Id()
					shortcuts = append(shortcuts, sc)
				}
			}
		}
		head, tail = pie.Shift(tail)
		hidx += 1
	}
	return shortcuts
}

// ---------- UTILITIES -----------------------------------

func (m *Maze) Initialize() {
	graph := dijkstra.NewGraph()
	vmap := make(map[string]int)
	ivmap := make(map[int]string)
	for pid := range m.points {
		idx := graph.AddNewEmptyVertex()
		vmap[pid] = idx
		ivmap[idx] = pid
	}
	for pid, p := range m.points {
		fnode := vmap[pid]
		for _, aid := range p.AdjacentIds() {
			_, ok := m.points[aid]
			if ok {
				tnode := vmap[aid]
				graph.AddArc(fnode, tnode, uint64(1))
			}
		}
	}
	onode := vmap[m.OriginId()]
	tnode := vmap[m.TerminusId()]
	best, _ := graph.Shortest(onode, tnode)
	path := make([]string, 0)
	for _, idx := range best.Path {
		path = append(path, ivmap[idx])
	}
	m.best_path = path
	m.best_distance = int(best.Distance)
}

func (m Maze) OriginId() string {
	oid := ""
	for id, p := range m.points {
		if p.IsOrigin() {
			oid = id
		}
	}
	return oid
}

func (m Maze) TerminusId() string {
	tid := ""
	for id, p := range m.points {
		if p.IsTerminus() {
			tid = id
		}
	}
	return tid
}

// ---------- VISUALS -------------------------------------

func (m Maze) Dimensions() (int, int, int, int) {
	min_x := 999999999
	max_x := 0
	min_y := 999999999
	max_y := 0
	for _, p := range m.points {
		switch {
		case p.x < min_x:
			min_x = p.x
		case p.x > max_x:
			max_x = p.x
		case p.y < min_y:
			min_y = p.y
		case p.y > max_y:
			max_y = p.y
		}
	}
	return min_x, max_x, min_y, max_y
}

func (m Maze) Print() {
	min_x, max_x, min_y, max_y := m.Dimensions()

	for y := min_y - 1; y <= max_y+1; y++ {
		row := ""
		for x := min_x - 1; x <= max_x+1; x++ {
			p, ok := m.points[utility.CoordToId(x, y)]
			if ok {
				row += p.value
			} else {
				row += "#"
			}
		}
		fmt.Println(row)
	}
	fmt.Println("")
}
