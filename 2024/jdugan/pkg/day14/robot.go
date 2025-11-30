package day14

import "aoc/2024/pkg/utility"

// ========== DEFINITION ==================================

type Robot struct {
	x  int
	y  int
	dx int
	dy int
}

// ========== RECEIVERS ===================================

func (r Robot) Id() string {
	return utility.CoordToId(r.x, r.y)
}

func (r Robot) AdjacentIds() []string {
	e := utility.CoordToId(r.x+1, r.y)
	n := utility.CoordToId(r.x, r.y-1)
	s := utility.CoordToId(r.x, r.y+1)
	w := utility.CoordToId(r.x-1, r.y)
	return []string{e, n, s, w}
}
