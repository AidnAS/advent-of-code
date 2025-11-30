package day01

import (
	"aoc/2024/pkg/reader"
	"aoc/2024/pkg/utility"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 1)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	lefts, rights := data()
	sort.Ints(lefts)
	sort.Ints(rights)
	sum := 0
	for i, left := range lefts {
		sum += utility.Distance(left, rights[i])
	}
	return sum
}

func Puzzle2() int {
	lefts, rights := data()
	rhash := listToMap(rights)
	sum := 0
	for _, k := range lefts {
		v := rhash[k]
		sum += k * v
	}
	return sum
}

// ========== PRIVATE FNS =================================

func data() ([]int, []int) {
	lines := reader.Lines("./data/day01/input.txt")
	lefts := make([]int, 0)
	rights := make([]int, 0)
	for _, line := range lines {
		nums := strings.Split(line, "   ")
		left, _ := strconv.Atoi(nums[0])
		right, _ := strconv.Atoi(nums[1])
		lefts = append(lefts, left)
		rights = append(rights, right)
	}
	return lefts, rights
}

func listToMap(list []int) map[int]int {
	hash := make(map[int]int)
	for _, val := range list {
		count := hash[val]
		hash[val] = count + 1
	}
	return hash
}
