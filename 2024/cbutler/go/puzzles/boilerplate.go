package puzzles

import (
	"aoc/utils"
	"fmt"
)

var Boilerplate = Puzzle{
	SolvePart1: func(date string, useTestData bool) {
		fmt.Println("Day 00, Part 1")

		lines, err := utils.DataFileReader(date, useTestData, "1", false)
		if err != nil {
			fmt.Println("Error reading file:", err)
			return
		}
		for _, line := range lines {
			fmt.Println(line)
		}

	},

	SolvePart2: func(date string, useTestData bool) {
		fmt.Println("Day 00, Part 2")
	},
}
