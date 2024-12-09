package day07

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 7)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	equations := data()
	operands := []string{"+", "*"}
	sum := 0
	for _, e := range equations {
		solutions := e.FindSolutions(operands)
		if len(solutions) > 0 {
			sum += e.result
		}
	}
	return sum
}

func Puzzle2() int {
	equations := data()
	operands := []string{"+", "*", "||"}
	sum := 0
	for _, e := range equations {
		solutions := e.FindSolutions(operands)
		if len(solutions) > 0 {
			sum += e.result
		}
	}
	return sum
}

// ========== PRIVATE FNS =================================

func data() []Equation {
	lines := reader.Lines("./data/day07/input.txt")
	equations := make([]Equation, 0)
	for _, line := range lines {
		strs := strings.Split(strings.Replace(line, ":", "", 1), " ")
		ints := pie.Ints(strs)
		result, factors := pie.Shift(ints)
		equations = append(equations, Equation{result: result, factors: factors})
	}
	return equations
}
