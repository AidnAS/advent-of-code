package day16

import (
	"aoc/2024/pkg/utility"
	"fmt"
	"strings"

	"github.com/RyanCarrier/dijkstra/v2"
	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Maze struct {
	points map[string]Point
}

// ========== RECEIVERS ===================================

func (m Maze) FindShortestPath() ([]string, int) {
	dirs := []string{"E", "N", "S", "W"}
	origin := m.GetOriginId()
	terminus := m.GetTerminusId()

	g := dijkstra.NewGraph()
	// add nodes and map ids to indices
	vmap := make(map[string]int)
	ivmap := make(map[int]string)
	for pid, _ := range m.points {
		for _, pdir := range dirs {
			vid := pid + ";" + pdir
			idx := g.AddNewEmptyVertex()
			vmap[vid] = idx
			ivmap[idx] = vid
		}
	}
	// add edges using map to translate ids to indices
	for pid, p := range m.points {
		amap := m.AdjacentMap(p)
		for _, pdir := range dirs {
			fid := pid + ";" + pdir
			fnode := vmap[fid]
			for adir, ap := range amap {
				tid := ap.Id() + ";" + adir
				tnode := vmap[tid]
				cost := 1
				if pdir != adir {
					cost = 1001
				}
				g.AddArc(fnode, tnode, uint64(cost))
			}
		}
	}

	best_seats := make([]string, 0)
	best_cost := 999999999999999
	oid := origin + ";E"
	onode := vmap[oid]
	for _, dir := range dirs {
		tid := terminus + ";" + dir
		tnode := vmap[tid]
		best, _ := g.ShortestAll(onode, tnode)
		cost := int(best.Distance)
		if cost > 0 {
			if cost < best_cost {
				best_seats = make([]string, 0)
			}
			for _, path := range best.Paths {
				for _, idx := range path {
					parts := strings.Split(ivmap[idx], ";")
					best_seats = append(best_seats, parts[0])
				}
			}
			best_cost = cost
		}
		best_seats = pie.Unique(best_seats)
	}
	return best_seats, best_cost
}

// ---------- UTILITIES -----------------------------------

func (m Maze) AdjacentMap(p Point) map[string]Point {
	amap := make(map[string]Point)
	// east
	id := p.EastId()
	ap, ok := m.points[id]
	if ok {
		amap["E"] = ap
	}
	// north
	id = p.NorthId()
	ap, ok = m.points[id]
	if ok {
		amap["N"] = ap
	}
	// south
	id = p.SouthId()
	ap, ok = m.points[id]
	if ok {
		amap["S"] = ap
	}
	// west
	id = p.WestId()
	ap, ok = m.points[id]
	if ok {
		amap["W"] = ap
	}
	return amap
}

func (m Maze) GetOriginId() string {
	oid := ""
	for id, p := range m.points {
		if p.IsOrigin() {
			oid = id
		}
	}
	return oid
}

func (m Maze) GetTerminusId() string {
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
