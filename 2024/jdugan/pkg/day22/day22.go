package day22

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strconv"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 22)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	market := data()
	return market.Checksum()
}

func Puzzle2() int {
	market := data()
	return market.BestDeal()
}

// ========== PRIVATE FNS =================================

func data() Market {
	lines := reader.Lines("./data/day22/input.txt")
	monkeys := make([]Monkey, 0)
	for _, line := range lines {
		num, _ := strconv.Atoi(line)
		monkeys = append(monkeys, Monkey{secret: num})
	}
	return Market{monkeys: monkeys}
}
