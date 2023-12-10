package day6

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type race struct {
	time int
	distance int
}

func Run() {
	file, _ := os.Open("cmd/solver/day6/input.txt")
	defer file.Close()
	solve(file);
}

func solve(file *os.File) {
	races := readRaces(file)
	score := calculateWinScore(races)
	
	fmt.Println("part 1", score)
	solvePartTwo(races)
}

func solvePartTwo(races []race) {
	timeValue, distanceValue := "", ""
	for _, race := range races {
		timeValue += strconv.Itoa(race.time)
		distanceValue += strconv.Itoa(race.distance)
	}
	time,_ := strconv.Atoi(timeValue)
	distance,_ := strconv.Atoi(distanceValue)
	score := calculateWinScore([]race{{ time: time, distance: distance }})
	fmt.Println("part 2:", score)
}

func readRaces(file *os.File) []race {
	scanner := bufio.NewScanner(file)
	re, raceDocument, i := regexp.MustCompile(`\d+`), make([][]int, 2), 0
	for scanner.Scan() {
		data := re.FindAllString(scanner.Text(), -1)		
		for _, d := range data {
			value, _ := strconv.Atoi(d)
			raceDocument[i] = append(raceDocument[i], value)
		}
		i++
	}
	races := make([]race, len(raceDocument[0]))
	for i := 0; i < len(races); i++ {
		races[i].time = raceDocument[0][i]
		races[i].distance = raceDocument[1][i]
	}
	return races
}

func calculateWinScore(races []race) int {
	holdTimes, score := getLowestWinningHoldTimesByBinarySearch(races), 1	 
	for i, holdTime := range holdTimes {
		score *= (races[i].time - 2*holdTime + 1)
	}
	return score
}

func getLowestWinningHoldTimesByBinarySearch(races []race) []int {
	holdTimes := make([]int, len(races))
	for i, race := range races {
		timeUpper, timeLower := race.time / 2, 0
		for timeLower != timeUpper {
			if timeUpper - timeLower > 1 {
				setNextHoldTimeSearchRange(&timeUpper, &timeLower, race)
				continue
			}
			holdTimes[i] = timeUpper
			timeUpper = timeLower
		}
	}
	return holdTimes
}

func setNextHoldTimeSearchRange(upper *int, lower *int, race race) {
	mid := (*upper + *lower) / 2
	distance := raceDistance(mid, race.time - mid)
	if distance > race.distance {
		*upper = mid
	} else {
		*lower = mid
	}
}

func raceDistance(holdTime int, raceTime int) int {
	return raceTime*holdTime
}