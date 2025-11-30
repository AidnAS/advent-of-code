package day14

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 14)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	room := data()
	room.ElapseTime(100)
	return room.SafetyFactor()
}

func Puzzle2() int {
	room := data()
	ticks := room.FirstChristmasTree()
	// room = data()
	// room.ElapseTime(ticks)
	// room.Print()
	return ticks
}

// ========== PRIVATE FNS =================================

func data() Room {
	lines := reader.Lines("./data/day14/input.txt")
	robots := make([]Robot, 0)
	dims := pie.Ints(strings.Split(lines[0], ","))
	for _, line := range lines[1:] {
		parts := strings.Split(line, " ")
		left := strings.Replace(parts[0], "p=", "", 1)
		coords := pie.Ints(strings.Split(left, ","))
		right := strings.Replace(parts[1], "v=", "", 1)
		deltas := pie.Ints(strings.Split(right, ","))

		robot := Robot{x: coords[0], y: coords[1], dx: deltas[0], dy: deltas[1]}
		robots = append(robots, robot)
	}
	return Room{width: dims[0], height: dims[1], robots: robots}
}
