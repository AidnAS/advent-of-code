package puzzles

import (
	"aoc/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

var Dec01 = Puzzle{
	SolvePart1: func(date string, useTestData bool) {
		fmt.Println("Day 01, Part 1")

		lines, err := utils.DataFileReader(date, useTestData, "1", false)
		if err != nil {
			fmt.Println("Error reading file:", err)
			return
		}

		// create two lists of Ids
		var firstIds []int
		var secondIds []int

		// add the first id to one list and the second id to another list
		for _, line := range lines {
			lineIds := strings.Fields(line)
			if len(lineIds) >= 2 {
				id1, err1 := strconv.Atoi(lineIds[0])
				id2, err2 := strconv.Atoi(lineIds[1])
				if err1 == nil && err2 == nil {
					firstIds = append(firstIds, id1)
					secondIds = append(secondIds, id2)
				} else {
					fmt.Println("Error converting string to int: ", err1, err2)
				}
			}
		}

		// sort both lists
		sort.Ints(firstIds)
		sort.Ints(secondIds)

		// Print the slices
		// fmt.Println("First IDs:", firstIds)
		// fmt.Println("Second IDs:", secondIds)

		var diffs []int

		// compare both lists and count
		if len(firstIds) != len(secondIds) {
			panic("Error: the two lists are not the same length")
		}

		for i := 0; i < len(firstIds); i++ {
			diff := firstIds[i] - secondIds[i]
			if diff < 0 {
				diff = -diff
			}
			diffs = append(diffs, diff)
		}

		// fmt.Println("Differences:", diffs)
		var total int = 0

		for i := 0; i < len(diffs); i++ {
			total += diffs[i]
		}

		var outputString = "Total [using puzzle data]: "

		if useTestData {
			outputString = "Total [using test data]: "
		}

		fmt.Println(outputString, total, "\n\n")

	}, // SolvePart1

	SolvePart2: func(date string, useTestData bool) {
		fmt.Println("Day 01, Part 2")

		lines, err := utils.DataFileReader(date, useTestData, "1", false)
		if err != nil {
			fmt.Println("Error reading file:", err)
			return
		}

		// create two lists of Ids
		var firstIds []int
		var secondIds []int

		// add the first id to one list and the second id to another list
		for _, line := range lines {
			lineIds := strings.Fields(line)
			if len(lineIds) >= 2 {
				id1, err1 := strconv.Atoi(lineIds[0])
				id2, err2 := strconv.Atoi(lineIds[1])
				if err1 == nil && err2 == nil {
					firstIds = append(firstIds, id1)
					secondIds = append(secondIds, id2)
				} else {
					fmt.Println("Error converting string to int: ", err1, err2)
				}
			}
		}

		// compare both lists and count
		if len(firstIds) != len(secondIds) {
			panic("Error: the two lists are not the same length")
		}

		// find the similarity scores for each id in the first list
		var scores []int

		for i := 0; i < len(firstIds); i++ {
			count := 0
			for j := 0; j < len(secondIds); j++ {
				if firstIds[i] == secondIds[j] {
					count++
				}
			}

			scores = append(scores, firstIds[i]*count)
		}

		// count the total
		var total int = 0

		for i := 0; i < len(scores); i++ {
			total += scores[i]
		}

		var outputString = "Total [using puzzle data]: "

		if useTestData {
			outputString = "Total [using test data]: "
		}

		fmt.Println(outputString, total, "\n\n")

	}, // SolvePart2
}
