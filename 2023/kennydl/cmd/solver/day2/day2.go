package day2

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func Run() {
	file, _ := os.Open("cmd/solver/day2/input.txt")
	defer file.Close()
	partOne(file)
}

type gameBag struct {
	red int
	green int
	blue int
}

func partOne(file *os.File) {	
	scanner := bufio.NewScanner(file)
	config := gameBag{red: 12, green: 13, blue: 14}
	result, sumPower, gameNumber := 0, 0, 1

	for scanner.Scan()	{
		line := scanner.Text()
		bags, powerScore := readGame(line)

		if(validate(config, bags)) {
			result += gameNumber
		}

		sumPower += powerScore
		gameNumber++
	}
	fmt.Println("part 1:", result)
	fmt.Println("part 2:", sumPower)
}

func readGame(line string) ([]gameBag, int) {
	bags := []gameBag{}
	games := strings.Split(line, ";")
	maxRed, maxGreen, maxBlue := 0, 0, 0

	for _, game := range games {
		bag := gameBag{
			red: getNumber(game, "red"),
			green: getNumber(game, "green"),
			blue: getNumber(game, "blue")}

		if bag.red > maxRed {
			maxRed = bag.red
		}
		if bag.green > maxGreen {
			maxGreen = bag.green
		}
		if bag.blue > maxBlue {		
			maxBlue = bag.blue
		}

		bags = append(bags, bag)
	}
	return bags, maxRed*maxGreen*maxBlue
}

func getNumber(bag string, color string) int {
	re := regexp.MustCompile(`(\d+) ` + color)
	match := re.FindAllStringSubmatch(bag, -1)
	if match != nil {
		value, _ := strconv.Atoi(match[0][1])
		return value
	}
	return 0
}

func validate(config gameBag, bags []gameBag) bool {
	for _, bag := range bags {
		if bag.red > config.red || bag.green > config.green || bag.blue > config.blue {
			return false
		}
	}
	return true
}