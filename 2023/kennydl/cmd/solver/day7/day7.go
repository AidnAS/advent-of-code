package day7

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type hand struct { cards string; bid int }
var allPossibleCards = []rune {'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'}

func Run() {
	file,_ := os.Open("cmd/solver/day7/input.txt")
	defer file.Close()
	solve(file);
}

func solve(file *os.File) {
	hands, totalWinnings := readHandSet(file), 0
	sort.Slice(hands, func(i, j int) bool { return sortByRank(hands, i, j, false)})	
	for i, hand := range hands { 
		totalWinnings += hand.bid * (i + 1)
	}
	fmt.Println("part 1:", totalWinnings)

	totalWinnings = 0
	sort.Slice(hands, func(i, j int) bool { return sortByRank(hands, i, j, true) })		
	for i, hand := range hands { 
		totalWinnings += hand.bid * (i + 1)
	}
	fmt.Println("part 2:", totalWinnings)
}

func sortByRank(hands []hand, i int, j int, isWildCardRule bool) bool {
	rankA,_ := strconv.Atoi(handRank(hands[i].cards, isWildCardRule) + totalCardRanks(hands[i].cards, isWildCardRule))
	rankB,_ := strconv.Atoi(handRank(hands[j].cards, isWildCardRule) + totalCardRanks(hands[j].cards, isWildCardRule))
	return rankA < rankB
}

func readHandSet (file *os.File) []hand {
	scanner := bufio.NewScanner(file)
	hands := make([]hand, 0)
	for scanner.Scan() {
		values := strings.Split(scanner.Text(), " ");
		bidValue,_ := strconv.Atoi(values[1])
		hands = append(hands, hand{ cards: values[0], bid: bidValue})
	}
	return hands
}

func handRank (cards string, isWildCardRule bool) string {
	var cardCount, uniqueCards, rank = make(map[rune]int), make(map[rune]rune), 0
	if isWildCardRule {
		cardCount, uniqueCards = handRankWildCardRule(cards)
	} else {
		cardCount, uniqueCards = handRankSimpleRule(cards)
	}
	
	for card := range uniqueCards {
		if cardCount[card] > rank {
			rank = cardCount[card]
		}
	}
	if len(uniqueCards) == 0 {
		uniqueCards['J'], rank = 'J', 5		 
	}
	return strconv.Itoa(rank - len(uniqueCards) + 5)
}

func handRankSimpleRule (cards string) (map[rune]int, map[rune]rune) {
	cardCount, uniqueCards := map[rune]int {}, map[rune]rune {}	 
	for _, card := range cards {
		cardCount[card]++
		uniqueCards[card] = card
	}
	return cardCount, uniqueCards
}

func handRankWildCardRule (cards string) (map[rune]int, map[rune]rune)  {
	cardCount, uniqueCards := map[rune]int {}, map[rune]rune {}	 
	for _, card := range cards {
		if card != 'J' {
			cardCount[card]++	
			uniqueCards[card] = card
			continue
		}
		for _, wildCard := range allPossibleCards {
			cardCount[wildCard]++
		}
	}
	return cardCount, uniqueCards
}

func totalCardRanks(cards string, isWildCardRule bool) string {
	totalCardRanks := ""
	for _, card := range cards {
		totalCardRanks += cardRank(card, isWildCardRule)
	}
	return totalCardRanks
}

func cardRank(card rune, isWildCardRule bool) string {
	switch card {
		case 'A':
			return "14"
		case 'K':
			return "13"
		case 'Q':
			return "12"
		case 'J':
			if isWildCardRule {
				return "01"
			}
			return "11"
		case 'T':
			return "10"
		default:
			return "0" + string(card)
	}
}
