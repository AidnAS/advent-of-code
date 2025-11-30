package day25

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"slices"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 25)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	keys, locks := data()
	count := 0
	for _, key := range keys {
		for _, lock := range locks {
			if key.Fits(lock) {
				count += 1
			}
		}
	}
	return count
}

func Puzzle2() int {
	return 50
}

// ========== PRIVATE FNS =================================

func data() ([]Key, []Lock) {
	lines := reader.Lines("./data/day25/input.txt")
	chunks := pie.Chunk(lines, 8)
	keys := make([]Key, 0)
	locks := make([]Lock, 0)
	for _, chunk := range chunks {
		if string(chunk[0][0]) == "." {
			key := parseKey(chunk[:7])
			keys = append(keys, key)
		} else {
			lock := parseLock(chunk[:7])
			locks = append(locks, lock)
		}
	}
	return keys, locks
}

func parseKey(chunk []string) Key {
	heights := parseHeights(chunk[1:6])
	return Key{notches: heights}
}

func parseLock(chunk []string) Lock {
	slices.Reverse(chunk)
	heights := parseHeights(chunk[1:6])
	return Lock{max: 5, pins: heights}
}

func parseHeights(chunk []string) []int {
	heights := make([]int, len(chunk[0]))
	for i, _ := range strings.Split(chunk[0], "") {
		heights[i] = 0
	}
	for _, line := range chunk {
		for i, r := range strings.Split(line, "") {
			if string(r) == "#" {
				heights[i] += 1
			}
		}
	}
	return heights
}
