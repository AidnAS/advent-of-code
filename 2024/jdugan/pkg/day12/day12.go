package day12

import (
	"aoc/2024/pkg/reader"
	"fmt"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 12)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	garden := data()
	garden.Divide()
	return garden.RegularCost()
}

func Puzzle2() int {
	garden := data()
	garden.Divide()
	return garden.BulkCost()
}

// ========== PRIVATE FNS =================================

func data() Garden {
	lines := reader.Lines("./data/day12/input.txt")
	dims := []int{len(lines[0]), len(lines)}
	points := make(map[string]Point)
	for y, row := range lines {
		for x, col := range row {
			p := Point{x: x, y: y, value: string(col)}
			points[p.Id()] = p
		}
	}
	return Garden{dims: dims, points: points}
}
