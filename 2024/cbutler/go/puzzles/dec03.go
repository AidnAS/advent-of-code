package puzzles

import (
	"aoc/utils"
	"fmt"
)

var Dec03 = Puzzle{
	SolvePart1: func(date string, useTestData bool) {
		fmt.Println("Day 02, Part 1")

		lines, err := utils.DataFileReader(date, useTestData, "1", false)
		if err != nil {
			fmt.Println("Error reading file:", err)
			return
		}
		for _, line := range lines {
			fmt.Println(line)
		}

		var total int = 0

		var outputString = "Total [using puzzle data]: "

		if useTestData {
			outputString = "Total [using test data]: "
		}

		fmt.Println(outputString, total, "\n\n")

	}, // SolvePart1

	SolvePart2: func(date string, useTestData bool) {
		fmt.Println("Day 02, Part 2")

		lines, err := utils.DataFileReader(date, useTestData, "1", false)
		if err != nil {
			fmt.Println("Error reading file:", err)
			return
		}
		for _, line := range lines {
			fmt.Println(line)
		}

		// count the total
		var total int = 0

		var outputString = "Total [using puzzle data]: "

		if useTestData {
			outputString = "Total [using test data]: "
		}

		fmt.Println(outputString, total, "\n\n")

	}, // SolvePart2
}
