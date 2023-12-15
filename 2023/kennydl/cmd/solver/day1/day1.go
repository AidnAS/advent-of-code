package day1

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func Run() {
	file, _ := os.Open("cmd/solver/day1/input.txt")
	defer file.Close()

	partOne(file)
	partTwo(file)
}

func partOne(file *os.File) {
	scanner := bufio.NewScanner(file)
	re, _ := regexp.Compile(`(\d)`)
	result := 0
	
	for scanner.Scan()	{
		line := scanner.Text()
		match := re.FindAllString(line, -1)
		if match != nil {
			value, _ := strconv.Atoi(getNumber(line, re))
			result += value
		}
	}

	fmt.Println("part 1:", result)
}

var numberValues = map[string]string{
	"one": "1",
	"two": "2",
	"three": "3",
	"four": "4",
	"five": "5",
	"six" : "6",
	"seven": "7",
	"eight": "8",
	"nine": "9"}

func partTwo(file *os.File) {
	file.Seek(0, 0)
	scanner := bufio.NewScanner(file)
	pattern := `(\d)`
	for k := range numberValues {
		pattern = pattern + `|(` + k + `)`
	}
	re, _ := regexp.Compile(pattern)
	result := 0

	for scanner.Scan()	{
		line := scanner.Text()
		value, _ := strconv.Atoi(getNumber(line, re))
		result += value
	}

	fmt.Println("part 2:", result)
}

func getNumber(line string, re *regexp.Regexp) string  {
	firstDigit, lastDigit, len := "", "", len(line)
	for i := range line {
		firstValue, lastValue := re.FindString(line[0:i+1]), re.FindString(line[len-1-i:len])
		if firstValue != "" && firstDigit == "" {
			firstDigit = convertToDigitString(firstValue)
		}
		if lastValue != "" && lastDigit == "" {
			lastDigit = convertToDigitString(lastValue)
		}
		if firstDigit != "" && lastDigit != "" {
			return firstDigit + lastDigit
		}
	}
	return ""
}

func convertToDigitString(value string) string  {
	digit := value
	_, err := strconv.Atoi(digit)

	if(err == nil) {
		return digit
	}
	return numberValues[value]
}