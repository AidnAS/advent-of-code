package day08

import (
	"aoc/2024/pkg/reader"
	"fmt"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 8)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	grid := data()
	antinodes := grid.MapSimpleAntiNodes()
	return len(antinodes)
}

func Puzzle2() int {
	grid := data()
	antinodes := grid.MapResonantAntiNodes()
	return len(antinodes)
}

// ========== PRIVATE FNS =================================

func data() Grid {
	lines := reader.Lines("./data/day08/input.txt")

	max_x := len(lines[0]) - 1
	max_y := len(lines) - 1
	points := make(map[string]Point)
	for y, row := range lines {
		for x, col := range row {
			if string(col) != "." {
				p := Point{x: x, y: y, value: string(col)}
				points[p.Id()] = p
			}
		}
	}
	return Grid{points: points, min_x: 0, max_x: max_x, min_y: 0, max_y: max_y}
}
