package day06

import "aoc/2024/pkg/utility"

// ========== DEFINITION ==================================

type Point struct {
	x     int
	y     int
	value string
}

// ========== RECEIVERS ===================================

// id stuff
func (p Point) Id() string {
	return utility.CoordToId(p.x, p.y)
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

// type stuff
func (p Point) MakeBlocked() Point {
	p.value = "#"
	return p
}

func (p Point) MakeOpen() Point {
	p.value = "."
	return p
}
