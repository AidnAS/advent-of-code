package day03

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strings"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 3)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	scanner := Scanner{program: data()}
	return scanner.SimpleParse()
}

func Puzzle2() int {
	scanner := Scanner{program: data()}
	return scanner.ComplexParse()
}

// ========== PRIVATE FNS =================================

func data() string {
	lines := reader.Lines("./data/day03/input.txt")

	return strings.Join(lines[:], "")
}
