package day10

import "aoc/2024/pkg/utility"

// ========== DEFINITION ==================================

type Point struct {
	x     int
	y     int
	value int
}

// ========== RECEIVERS ===================================

// id stuff
func (p Point) Id() string {
	return utility.CoordToId(p.x, p.y)
}

func (p Point) AdjacentIds() []string {
	ids := make([]string, 0)
	ids = append(ids, p.EastId())
	ids = append(ids, p.NorthId())
	ids = append(ids, p.SouthId())
	ids = append(ids, p.WestId())
	return ids
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
