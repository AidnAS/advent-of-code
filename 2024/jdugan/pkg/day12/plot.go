package day12

import (
	"aoc/2024/pkg/utility"
	"slices"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Plot struct {
	name      string
	ids       []string
	edges     [][]string
	area      int
	perimeter int
}

// ========== RECEIVERS ===================================

func (p Plot) BulkCost() int {
	return p.Area() * p.Sides()
}

func (p Plot) RegularCost() int {
	return p.Area() * p.Perimeter()
}

// ---------- UTILITY -------------------------------------

func (p Plot) Area() int {
	return len(p.ids)
}

func (p Plot) Borders() [][]string {
	edge, edges := pie.Shift(p.edges)
	border := []string{edge[0], edge[1]}
	borders := make([][]string, 0)
	for len(edges) > 0 {
		idx := slices.IndexFunc(edges, func(e []string) bool {
			return e[0] == edge[1]
		})
		if idx > -1 {
			edge = edges[idx]
			border = append(border, edge[1])
			edges = slices.Delete(edges, idx, idx+1)
		} else {
			borders = append(borders, border)
			edge, edges = pie.Shift(edges)
			border = []string{edge[0], edge[1]}
		}
	}
	borders = append(borders, border)
	return borders
}

func (p Plot) Direction(id1 string, id2 string) string {
	x1, y1 := utility.CoordFromId(id1)
	x2, y2 := utility.CoordFromId(id2)
	dir := "X"
	switch {
	case x1 < x2 && y1 == y2:
		dir = "E"
	case x1 == x2 && y1 > y2:
		dir = "N"
	case x1 == x2 && y1 < y2:
		dir = "S"
	case x1 > x2 && y1 == y2:
		dir = "W"
	}
	return dir
}

func (p Plot) Perimeter() int {
	return len(p.edges)
}

func (p Plot) Sides() int {
	sides := 0
	borders := p.Borders()
	for _, vertices := range borders {
		head, tail := pie.Shift(vertices)
		prev := head
		init_dir := ""
		dir := ""
		for len(tail) > 0 {
			head, tail = pie.Shift(tail)
			new_dir := p.Direction(prev, head)
			if new_dir != dir {
				if dir == "" {
					init_dir = new_dir
				}
				sides += 1
				dir = new_dir
			}
			prev = head
		}
		if init_dir == dir {
			sides -= 1
		}
	}
	return sides
}
