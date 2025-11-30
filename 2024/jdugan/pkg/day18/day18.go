package day18

import (
	"aoc/2024/pkg/reader"
	"aoc/2024/pkg/utility"
	"fmt"
	"strconv"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 18)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	grid, drops := data()
	dist := grid.FindDebugDistance(drops)
	return dist
}

func Puzzle2() string {
	grid, drops := data()
	byte := grid.FindTerminalByte(drops)
	return byte
}

// ========== PRIVATE FNS =================================

func data() (Grid, int) {
	lines := reader.Lines("./data/day18/input.txt")
	max, _ := strconv.Atoi(lines[0])
	drops, _ := strconv.Atoi(lines[1])
	bytes := lines[3:]
	points := make(map[string]Point)

	for y := 0; y <= max; y++ {
		for x := 0; x <= max; x++ {
			id := utility.CoordToId(x, y)
			points[id] = Point{x: x, y: y, value: "."}
		}
	}

	return Grid{max: max, points: points, bytes: bytes}, drops
}
