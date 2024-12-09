package day02

import (
	"fmt"
	"strconv"
	"strings"

	"aoc/2024/pkg/reader"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 2)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	reports := data()
	count := 0
	for _, report := range reports {
		if report.IsSafe() {
			count += 1
		}
	}
	return count
}

func Puzzle2() int {
	reports := data()
	count := 0
	for _, report := range reports {
		if report.IsSafe() || report.IsSortaSafe() {
			count += 1
		}
	}
	return count
}

// ========== PRIVATE FNS =================================

func data() []Report {
	lines := reader.Lines("./data/day02/input.txt")
	reports := make([]Report, 0)
	for _, line := range lines {
		strs := strings.Split(line, " ")
		vals := make([]int, 0)
		for _, str := range strs {
			val, _ := strconv.Atoi(str)
			vals = append(vals, val)
		}
		reports = append(reports, Report{values: vals})
	}
	return reports
}
