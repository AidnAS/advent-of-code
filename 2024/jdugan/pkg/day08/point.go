package day08

import "aoc/2024/pkg/utility"

// ========== DEFINITION ==================================

type Point struct {
	x     int
	y     int
	value string
}

// ========== RECEIVERS ===================================

func (p Point) Id() string {
	return utility.CoordToId(p.x, p.y)
}
