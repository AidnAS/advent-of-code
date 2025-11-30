package day19

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strings"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 19)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	towels, designs := data()
	sum := 0
	for _, design := range designs {
		if design.IsPossible(towels) {
			sum += 1
		}
	}
	return sum
}

func Puzzle2() int {
	towels, designs := data()
	sum := 0
	for _, design := range designs {
		sum += design.SequenceCount(towels)
	}
	return sum
}

// ========== PRIVATE FNS =================================

func data() (map[string][]string, []Design) {
	lines := reader.Lines("./data/day19/input.txt")

	towels := make(map[string][]string)
	for _, towel := range strings.Split(lines[0], ", ") {
		k := string(towel[0])
		list, ok := towels[k]
		if !ok {
			list = make([]string, 0)
		}
		list = append(list, towel)
		towels[k] = list
	}

	designs := make([]Design, 0)
	for _, pattern := range lines[2:] {
		designs = append(designs, Design{pattern: pattern})
	}

	return towels, designs
}
