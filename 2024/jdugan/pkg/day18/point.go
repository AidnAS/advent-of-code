package day18

import "aoc/2024/pkg/utility"

// ========== DEFINITION ==================================

type Point struct {
	x     int
	y     int
	value string
}

// ========== RECEIVERS ===================================

// identity
func (p Point) Id() string {
	return utility.CoordToId(p.x, p.y)
}

// neighbors
func (p Point) AdjacentIds() []string {
	e := p.EastId()
	n := p.NorthId()
	s := p.SouthId()
	w := p.WestId()
	return []string{e, n, s, w}
}

func (p Point) EastId() string {
	return utility.CoordToId(p.x+1, p.y)
}

func (p Point) NorthId() string {
	return utility.CoordToId(p.x, p.y-1)
}

func (p Point) SouthId() string {
	return utility.CoordToId(p.x, p.y+1)
}

func (p Point) WestId() string {
	return utility.CoordToId(p.x-1, p.y)
}

// state
func (p Point) IsCorrupted() bool {
	return p.value == "#"
}
