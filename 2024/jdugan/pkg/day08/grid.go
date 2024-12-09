package day08

import (
	"aoc/2024/pkg/utility"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Grid struct {
	min_x  int
	max_x  int
	min_y  int
	max_y  int
	points map[string]Point
}

// ========== RECEIVERS ===================================

func (g Grid) MapNodes() map[string][]string {
	nodes := make(map[string][]string, 0)
	for pid, p := range g.points {
		_, ok := nodes[p.value]
		if !ok {
			nodes[p.value] = make([]string, 0)
		}
		nodes[p.value] = append(nodes[p.value], pid)
	}
	return nodes
}

func (g Grid) MapResonantAntiNodes() map[string][]string {
	antinodes := make(map[string][]string, 0)
	nodes := g.MapNodes()
	for nid, pids := range nodes {
		head, tail := pie.Shift(pids)
		for len(tail) > 0 {
			p1 := g.points[head]
			for _, pid := range tail {
				p2 := g.points[pid]
				// find slope
				dx := p1.x - p2.x
				dy := p1.y - p2.y
				// add antinodes
				antinodes, _ = g.addAntiNode(antinodes, nid, p1.x, p1.y)
				antinodes = g.addAntiNodes(antinodes, nid, p1.x, p1.y, dx, dy, g.max_x)
				antinodes = g.addAntiNodes(antinodes, nid, p1.x, p1.y, -dx, -dy, g.max_x)
			}
			head, tail = pie.Shift(tail)
		}
	}
	return antinodes
}

func (g Grid) MapSimpleAntiNodes() map[string][]string {
	antinodes := make(map[string][]string, 0)
	nodes := g.MapNodes()
	for nid, pids := range nodes {
		head, tail := pie.Shift(pids)
		for len(tail) > 0 {
			p1 := g.points[head]
			for _, pid := range tail {
				p2 := g.points[pid]
				// find slope
				dx := p1.x - p2.x
				dy := p1.y - p2.y
				// add antinodes
				antinodes = g.addAntiNodes(antinodes, nid, p1.x, p1.y, dx, dy, 1)
				antinodes = g.addAntiNodes(antinodes, nid, p2.x, p2.y, -dx, -dy, 1)
			}
			head, tail = pie.Shift(tail)
		}
	}
	return antinodes
}

// ---------- UTILITIES -----------------------------------

func (g Grid) addAntiNode(antinodes map[string][]string, nid string, ax int, ay int) (map[string][]string, bool) {
	added := false
	if ax >= 0 && ax <= g.max_x && ay >= 0 && ay <= g.max_y {
		aid := utility.CoordToId(ax, ay)
		_, ok := antinodes[aid]
		if !ok {
			antinodes[aid] = make([]string, 0)
		}
		antinodes[aid] = append(antinodes[aid], nid)
		added = true
	}
	return antinodes, added
}

func (g Grid) addAntiNodes(antinodes map[string][]string, nid string, x int, y int, dx int, dy int, max int) map[string][]string {
	count := 0
	added := true
	for added && count < max {
		x += dx
		y += dy
		antinodes, added = g.addAntiNode(antinodes, nid, x, y)
		count += 1
	}
	return antinodes
}
