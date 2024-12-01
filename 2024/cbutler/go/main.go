package main

import (
	"aoc/puzzles"
	"fmt"
	"time"
)

func main() {
	currentDate := time.Now().Format("02-01-2006")
	fmt.Printf("AoC: %s\n", currentDate)
	fmt.Println("-----------------")

	// test boilerplate
	// Note - Go does not suport default values or optional parameters.
	// testDate := "01-12-2000"
	// puzzles.Boilerplate.SolvePart1(testDate, true)
	// puzzles.Boilerplate.SolvePart2(testDate, true)

	// run puzzle solvers
	puzzles.Dec01.SolvePart1(currentDate, false) // ans: 3508942
	puzzles.Dec01.SolvePart2(currentDate, false) // ans: 26593248
}
