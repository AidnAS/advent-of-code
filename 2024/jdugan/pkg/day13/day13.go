package day13

import (
	"aoc/2024/pkg/reader"
	"aoc/2024/pkg/utility"
	"fmt"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 13)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	machines := data(0)
	sum := 0
	for _, m := range machines {
		sum += m.FewestCoins(3, 1)
	}
	return sum
}

func Puzzle2() int {
	machines := data(10000000000000)
	sum := 0
	for _, m := range machines {
		sum += m.FewestCoins(3, 1)
	}
	return sum
}

// ========== PRIVATE FNS =================================

func data(offset int) []Machine {
	lines := reader.Lines("./data/day13/input.txt")
	machines := make([]Machine, 0)
	for _, chunk := range pie.Chunk(lines, 4) {
		ax, ay := parseInts(chunk[0], "Button A: ")
		bx, by := parseInts(chunk[1], "Button B: ")
		px, py := parseInts(chunk[2], "Prize: ")
		machines = append(machines, Machine{ax: ax, ay: ay, bx: bx, by: by, px: px + offset, py: py + offset})
	}
	return machines
}

func parseInts(str string, heading string) (int, int) {
	str = strings.Replace(str, heading, "", 1)
	str = strings.Replace(str, "X", "", 1)
	str = strings.Replace(str, "Y", "", 1)
	str = strings.Replace(str, "+", "", -1)
	str = strings.Replace(str, "=", "", -1)
	str = strings.Replace(str, " ", "", -1)
	return utility.CoordFromId(str)
}
