package day20

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 20)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	maze, config, _ := data()
	return maze.ShortcutCount(config.cheats, config.saving)
}

// 554470
func Puzzle2() int {
	maze, _, config := data()
	return maze.ShortcutCount(config.cheats, config.saving)
	// return -2
}

// ========== PRIVATE FNS =================================

func data() (Maze, Config, Config) {
	lines := reader.Lines("./data/day20/input.txt")

	// build config objects
	p1_cvals := pie.Ints(strings.Split(lines[0], ";"))
	p1_config := Config{cheats: p1_cvals[0], saving: p1_cvals[1]}
	p2_cvals := pie.Ints(strings.Split(lines[1], ";"))
	p2_config := Config{cheats: p2_cvals[0], saving: p2_cvals[1]}

	// build maze
	points := make(map[string]Point)
	for y, line := range lines[2:] {
		for x, col := range line {
			if string(col) != "#" {
				p := Point{x: x, y: y, value: string(col)}
				points[p.Id()] = p
			}
		}
	}
	maze := Maze{points: points}
	maze.Initialize()

	return maze, p1_config, p2_config
}
