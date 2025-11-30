package day12

import (
	"aoc/2024/pkg/utility"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Garden struct {
	dims   []int
	points map[string]Point
	plots  []Plot
}

// ========== RECEIVERS ===================================

func (g *Garden) Divide() {
	unvisited := g.PointIds()
	visited := make([]string, 0)
	plots := make([]Plot, 0)

	for len(unvisited) > 0 {
		rp, _ := g.points[unvisited[0]]
		name := rp.value
		ids := make([]string, 0)
		edges := make([][]string, 0)

		checkids := []string{rp.Id()}
		for len(checkids) > 0 {
			new_checkids := make([]string, 0)
			for _, cid := range checkids {
				cp, _ := g.points[cid]
				ids = append(ids, cid)
				// east
				id := cp.EastId()
				ap, ok := g.points[id]
				if ok && ap.value == name {
					if !pie.Contains(visited, id) {
						new_checkids = append(new_checkids, id)
					}
				} else {
					edges = append(edges, []string{utility.CoordToId(cp.x+1, cp.y), utility.CoordToId(cp.x+1, cp.y+1)})
				}
				// north
				id = cp.NorthId()
				ap, ok = g.points[id]
				if ok && ap.value == name {
					if !pie.Contains(visited, id) {
						new_checkids = append(new_checkids, id)
					}
				} else {
					edges = append(edges, []string{utility.CoordToId(cp.x, cp.y), utility.CoordToId(cp.x+1, cp.y)})
				}
				// south
				id = cp.SouthId()
				ap, ok = g.points[id]
				if ok && ap.value == name {
					if !pie.Contains(visited, id) {
						new_checkids = append(new_checkids, id)
					}
				} else {
					edges = append(edges, []string{utility.CoordToId(cp.x+1, cp.y+1), utility.CoordToId(cp.x, cp.y+1)})
				}
				// west
				id = cp.WestId()
				ap, ok = g.points[id]
				if ok && ap.value == name {
					if !pie.Contains(visited, id) {
						new_checkids = append(new_checkids, id)
					}
				} else {
					edges = append(edges, []string{utility.CoordToId(cp.x, cp.y+1), utility.CoordToId(cp.x, cp.y)})
				}
				visited = append(visited, cid)
			}
			checkids = pie.Unique(new_checkids)
		}
		unvisited, _ = pie.Diff(visited, unvisited)
		plot := Plot{name: name, ids: ids, edges: edges}
		plots = append(plots, plot)
	}
	g.plots = plots
}

func (g Garden) BulkCost() int {
	sum := 0
	for _, p := range g.plots {
		sum += p.BulkCost()
	}
	return sum
}

func (g Garden) RegularCost() int {
	sum := 0
	for _, p := range g.plots {
		sum += p.RegularCost()
	}
	return sum
}

// ---------- UTILITY -------------------------------------

func (g Garden) PointIds() []string {
	ids := make([]string, 0)
	for _, p := range g.points {
		ids = append(ids, p.Id())
	}
	return ids
}
