package day06

import (
	"aoc/2024/pkg/reader"
	"aoc/2024/pkg/utility"
	"fmt"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 6)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	guard, points := data()
	visits := guard.PatrolMap(points)
	return len(visits)
}

func Puzzle2() int {
	guard, points := data()
	origin := guard.current.position
	visits := guard.PatrolMap(points)
	delete(visits, origin)

	count := 0
	for vid, _ := range visits {
		p, _ := points[vid]
		points[vid] = p.MakeBlocked()
		guard = Guard{current: State{position: origin, direction: "N"}}
		_, looped := guard.Patrol(points)
		if looped {
			count += 1
		}
		points[vid] = p.MakeOpen()
	}
	return count
}

// ========== PRIVATE FNS =================================
func data() (Guard, map[string]Point) {
	lines := reader.Lines("./data/day06/input.txt")

	id := ""
	dir := "N"
	points := make(map[string]Point)
	for y, row := range lines {
		for x, col := range row {
			p := Point{x: x, y: y, value: string(col)}
			if string(col) == "^" {
				id = utility.CoordToId(x, y)
			}
			points[p.Id()] = p
		}
	}
	state := State{position: id, direction: dir}
	return Guard{current: state}, points
}
