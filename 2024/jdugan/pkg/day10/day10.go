package day10

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strconv"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 10)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	terrain := data()
	return terrain.GetTrailSystemScore()
}

func Puzzle2() int {
	terrain := data()
	return terrain.GetTrailSystemRating()
}

// ========== PRIVATE FNS =================================

func data() Terrain {
	lines := reader.Lines("./data/day10/input.txt")
	terrain := Terrain{points: make(map[string]Point)}
	for y, row := range lines {
		for x, col := range row {
			v, _ := strconv.Atoi(string(col))
			p := Point{x: x, y: y, value: v}
			terrain.UpdatePoint(p)
		}
	}
	return terrain
}
