package day4

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
	"sync"
)

type scratchCard struct {
	nr int
	numbers []string
	winningNumbers []string
	totalOfWinningNumbers int
	points int
	totalCopies int
}

type concurrentMap struct {
	m   map[int]scratchCard
	mux sync.RWMutex
}

func newConcurrentMap() *concurrentMap {
	return &concurrentMap {
		m:   make(map[int]scratchCard),
		mux: sync.RWMutex{},
	}
}

func Run() {
	file, _ := os.Open("cmd/solver/day4/input.txt")
	defer file.Close()
	solve(file);
}

func solve(file *os.File) {
	scanner := bufio.NewScanner(file)
	
	var wg sync.WaitGroup
	scratchCards := newConcurrentMap()
	for scanner.Scan()	{
		wg.Add(1)
		go calculatePoints(&wg, scratchCards, scanner.Text())
	}
	wg.Wait()

	result, totalScratchCards := 0, 0
	for i := 1; i <= len(scratchCards.m); i++ {
		card := scratchCards.m[i]		
		result += card.points
		calculateCopies(card, scratchCards)
		totalScratchCards += card.totalCopies
	}

	fmt.Println("part 1:", result)
	fmt.Println("part 2:", totalScratchCards)
}

// Part 1
func calculatePoints(wg *sync.WaitGroup, result *concurrentMap, line string) {
	defer wg.Done()
	defer result.mux.Unlock()
	result.mux.Lock()
	scratchCard := readCard(line)

	for _, number := range scratchCard.numbers {
		for _, winningNumber := range scratchCard.winningNumbers {
			if number == winningNumber {
				scratchCard.totalOfWinningNumbers += 1
			}
		}
	}

	if scratchCard.totalOfWinningNumbers > 0 {
		scratchCard.points = 1 << (scratchCard.totalOfWinningNumbers - 1)
	}
	result.m[scratchCard.nr] = scratchCard	
}

// Part 2
func calculateCopies(card scratchCard, result *concurrentMap) {
	for j := 1; j <= card.totalOfWinningNumbers; j++ {
		next := result.m[card.nr+j]
		next.totalCopies += card.totalCopies
		result.m[card.nr+j] = next
	}
}

func readCard(line string) scratchCard {
	re, reWhiteSpace := regexp.MustCompile(`(\d+):\s+(\d+.+\d+)\s+\|\s+(\d+.+\d+)`), regexp.MustCompile(`\s+`)
	match := re.FindStringSubmatch(line)
	cardNr,_ := strconv.Atoi(match[1])

	return scratchCard {
		nr: cardNr,
		numbers: strings.Split(reWhiteSpace.ReplaceAllString(match[2], ";"), ";"),
		winningNumbers: strings.Split(reWhiteSpace.ReplaceAllString(match[3], ";"), ";"),
		totalOfWinningNumbers: 0,
		points: 0,
		totalCopies: 1,
	}
}