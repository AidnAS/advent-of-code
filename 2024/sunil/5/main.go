package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	readFile, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
	}

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	printOrder := map[int][]int{}

	finishedReadingOrder := false
	sum := 0
	incorrectOrderSum := 0

	// TODO: 1. Use string splitter to read different lines
	// 2. dont convert to int use string all the way except at line 58
	// 3. merge the functions with a boolean
	for fileScanner.Scan() {
		line := fileScanner.Text()
		if line == "" {
			fmt.Printf("input order - %+v\n", printOrder)
			finishedReadingOrder = true
			continue
		}
		if !finishedReadingOrder {
			order := strings.Split(line, "|")
			orderPri1, _ := strconv.Atoi(order[0])
			orderPri2, _ := strconv.Atoi(order[1])

			if printOrder[orderPri1] == nil {
				printOrder[orderPri1] = make([]int, 0)
			}
			printOrder[orderPri1] = append(printOrder[orderPri1], orderPri2)
		} else {

			// read the input
			printingInput := strings.Split(line, ",")
			update := make([]int, 0)
			for _, input := range printingInput {
				num, _ := strconv.Atoi(input)
				update = append(update, num)
			}

			fmt.Printf("read the input to numbers - %+v\n", update)

			if isUpdateCorrect(printOrder, update) {
				sum += update[len(update)/2]
			} else {
				// fix it
				updatedList := fixUpdate(printOrder, update)
				incorrectOrderSum += updatedList[len(updatedList)/2]
			}
		}
	}

	fmt.Printf("Sum = %d\n", sum)
	fmt.Printf("Incorrect Update Sum = %d\n", incorrectOrderSum)
	readFile.Close()
}

func isUpdateCorrect(rules map[int][]int, update []int) bool {
	for i, num := range update {
		for _, next := range update[i+1:] {
			if !isNumInSlice(next, rules[num]) {
				return false
			}
		}
	}
	return true
}

func fixUpdate(rules map[int][]int, update []int) []int {
	for i := 0; i < len(update)-1; i++ {
		for j := i + 1; j < len(update); j++ {
			if rules[update[j]] != nil && isNumInSlice(update[i], rules[update[j]]) {
				update[i], update[j] = update[j], update[i]
				return fixUpdate(rules, update)
			}
		}
	}
	return update
}

func isNumInSlice(num int, slice []int) bool {
	for _, n := range slice {
		if n == num {
			return true
		}
	}
	return false
}
