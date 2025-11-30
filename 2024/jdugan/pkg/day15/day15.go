package day15

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strings"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 15)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	wh := data()
	wh.PerformMoves()
	return wh.GpsScore()
}

func Puzzle2() int {
	wh := data()
	wh.Expand()
	wh.PerformMoves()
	return wh.GpsScore()
}

// ========== PRIVATE FNS =================================

func data() Warehouse {
	lines := reader.Lines("./data/day15/input.txt")
	moves := make([]string, 0)
	points := make(map[string]Point)

	for y, line := range lines {
		switch {
		case strings.Index(line, "#") > -1:
			for x, col := range line {
				p := Point{x: x, y: y, value: string(col)}
				points[p.Id()] = p
			}
		default:
			strs := strings.Split(line, "")
			moves = append(moves, strs...)
		}
	}

	return Warehouse{moves: moves, points: points}
}
