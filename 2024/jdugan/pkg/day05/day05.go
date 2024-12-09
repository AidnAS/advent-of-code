package day05

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strconv"
	"strings"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 5)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	rules, runs := data()
	sum := 0
	for _, run := range runs {
		ok, checksum := run.IsValid(rules)
		if ok {
			sum += checksum
		}
	}
	return sum
}

func Puzzle2() int {
	rules, runs := data()
	sum := 0
	for _, run := range runs {
		ok, _ := run.IsValid(rules)
		if !ok {
			run.Repair(rules)
			sum += run.checksum
		}
	}
	return sum
}

// ========== PRIVATE FNS =================================

func data() ([]Rule, []PrintRun) {
	lines := reader.Lines("./data/day05/input.txt")
	rules := make([]Rule, 0)
	runs := make([]PrintRun, 0)

	for _, line := range lines {
		switch {
		case strings.Index(line, "|") > -1:
			strs := strings.Split(line, "|")
			str1, _ := strconv.Atoi(strs[0])
			str2, _ := strconv.Atoi(strs[1])
			rules = append(rules, Rule{before: str1, after: str2})
		case strings.Index(line, ",") > -1:
			pages := make([]int, 0)
			strs := strings.Split(line, ",")
			for _, str := range strs {
				num, _ := strconv.Atoi(str)
				pages = append(pages, num)
			}
			idx := len(pages) / 2
			runs = append(runs, PrintRun{pages: pages, checksum: pages[idx]})
		}
	}

	return rules, runs
}
