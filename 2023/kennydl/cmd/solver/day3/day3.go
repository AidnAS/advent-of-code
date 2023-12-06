package day3

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func Run() {
	file, _ := os.Open("cmd/solver/day3/input.txt")
	defer file.Close()
	solve(file)
}

func solve(file *os.File) {
	scanner := bufio.NewScanner(file)
	previous, current, next, gearPartSum, gearRatioSum := "", "", "", 0, 0

	for scanner.Scan()	{
		previous, current, next = current, next, scanner.Text()
		gearPartSum += getSumOfPartNumbers(previous, current, next);
		gearRatioSum += getGearRatio(previous, current, next);
	}
	gearPartSum += getSumOfPartNumbers(current, next, "");

	fmt.Println("part 1:", gearPartSum)
	fmt.Println("part 2:", gearRatioSum)
}

func getGearRatio(previous string, current string, next string) int {
	gearRatioSymbols := regexp.MustCompile(`\*`)
	reNumbers := regexp.MustCompile(`\d+`)

	numberIndexes := append(
		append(
			reNumbers.FindAllStringIndex(previous, -1),
			reNumbers.FindAllStringIndex(current, -1)...), 
		reNumbers.FindAllStringIndex(next, -1)...)
	numbers := append(
		append(
			reNumbers.FindAllString(previous, -1),
			reNumbers.FindAllString(current, -1)...),
		reNumbers.FindAllString(next, -1)...)

	sum := 0
	for _, symbolIndexes := range gearRatioSymbols.FindAllStringIndex(current, -1) {
		adjacentIndexes := getAdjecentNumberIndexes(symbolIndexes, numberIndexes)
		if len(adjacentIndexes) == 2 {
			firstValue,_ := strconv.Atoi(numbers[adjacentIndexes[0]])
			secondValue,_ := strconv.Atoi(numbers[adjacentIndexes[1]])
			sum += firstValue*secondValue
		}
	}
	return sum
}

func getAdjecentNumberIndexes(symbolIndexes []int, adjecentIndexes [][]int) []int {
	result := []int{}
	for i, indexes := range adjecentIndexes {
		if symbolIndexes[0] >= indexes[0]-1  && symbolIndexes[1] <= indexes[1]+1 {
			result = append(result, i)
		}
	}
	return result
}

func getSumOfPartNumbers(previous string, current string, next string) int {
	reSymbols := regexp.MustCompile(`[^0-9.]`)
	reNumbers := regexp.MustCompile(`\d+`)

	numberIndexes := reNumbers.FindAllStringIndex(current, -1)
	numbers := reNumbers.FindAllString(current, -1)

	symbolIndexes := append(
		append(
			reSymbols.FindAllStringIndex(previous, -1), 
			reSymbols.FindAllStringIndex(current, -1)...), 
		reSymbols.FindAllStringIndex(next, -1)...)

	sum := 0
	for i, indexs := range numberIndexes {
		if isAdjecentSymbol(indexs, symbolIndexes) {
			value,_ := strconv.Atoi(numbers[i])
			sum += value
		}
	}
	return sum
}

func isAdjecentSymbol(numberIndexes []int, adjecentIndexes [][]int) bool {
	for _, indexes := range adjecentIndexes {
		if indexes[0]+1 >= numberIndexes[0] && indexes[1]-1 <= numberIndexes[1] {
			return true;
		}
	}
	return false
}