package day15

import "aoc/2024/pkg/utility"

// ========== DEFINITION ==================================

type Point struct {
	x     int
	y     int
	value string
}

// ========== RECEIVERS ===================================

// ids
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

// scores
func (p Point) GpsScore() int {
	score := 0
	if p.IsBox() {
		score += (100 * p.y) + p.x
	}
	return score
}

// types
func (p Point) IsBox() bool {
	return p.value == "O" || p.value == "["
}

func (p Point) IsOpen() bool {
	return p.value == "."
}

func (p Point) IsRobot() bool {
	return p.value == "@"
}

func (p Point) IsWall() bool {
	return p.value == "#"
}
