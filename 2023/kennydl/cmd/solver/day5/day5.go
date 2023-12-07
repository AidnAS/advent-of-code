package day5

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type mapping struct {
	destinationStart int
	sourceStart int
	rangeLength int
	mapsToDestination int
}

func Run() {
	file, _ := os.Open("cmd/solver/day5/input.txt")
	defer file.Close()
	solve(file);
}

func solve(file *os.File) {
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	seeds := readSeeds(scanner.Text())
	mappings := make(map[int][]mapping)
	
	source := -1
	for scanner.Scan()	{
		line := scanner.Text()
		if( strings.Contains(line, ":")) {
			source++
			continue
		}
		readMapping(&mappings, line, source)
	}

	minLocation := math.MaxInt32
	for _, seed := range seeds {
		location := getDesiredDestination(seed, mappings, 0, 6)
		if location < minLocation {
			minLocation = location
		}
	}
	fmt.Println("part 1:", minLocation)

	minLocation = math.MaxInt32
	for i := 0; i < len(seeds); i += 2  {		
		for seed := seeds[i]; seed < seeds[i]+seeds[i+1]; seed++  {			
			location := getDesiredDestination(seed, mappings, 0, 6)
			if location < minLocation {
				minLocation = location
			}
		}
	}
	fmt.Println("part 2:", minLocation)
}

func readSeeds(line string) []int {
	re := regexp.MustCompile(`:\s+(\d+.+\d+)`)
	match := re.FindStringSubmatch(line)
	seeds := []int{}
	for _,seed := range strings.Split(match[1], " ") {
		value,_ := strconv.Atoi(seed)
		seeds = append(seeds, value)
	}
	return seeds
}

func readMapping(mappings *map[int][]mapping, line string, source int) {
	re := regexp.MustCompile(`(\d+) (\d+) (\d+)`)
	match := re.FindStringSubmatch(line)
	if match != nil {
		destinationStart,_ := strconv.Atoi(match[1])
		sourceStart,_ := strconv.Atoi(match[2])
		rangeLength,_ := strconv.Atoi(match[3])
		mapping := mapping{destinationStart, sourceStart, rangeLength, source + 1}
		(*mappings)[source] = append((*mappings)[source], mapping)
	}
}

func getDesiredDestination(source int, mappings map[int][]mapping, currentDestination int, desiredDestination int) int {
	for _,mapping := range mappings[currentDestination] {
		if source >= mapping.sourceStart && source < mapping.sourceStart + mapping.rangeLength {
			newSource := mapping.destinationStart + source - mapping.sourceStart
			if (currentDestination < desiredDestination) {
				return getDesiredDestination(newSource, mappings, mapping.mapsToDestination, desiredDestination)
			}
			return newSource
		}
	}
	if (currentDestination < desiredDestination) {
		return getDesiredDestination(source, mappings, currentDestination + 1, desiredDestination)
	}
	return source
}