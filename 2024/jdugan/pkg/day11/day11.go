package day11

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strconv"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 11)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	stones := data()
	stones = blink(stones, 25)
	return pie.Sum(pie.Values(stones))
}

func Puzzle2() int {
	stones := data()
	stones = blink(stones, 75)
	return pie.Sum(pie.Values(stones))
}

// ========== PRIVATE FNS =================================

func data() map[int]int {
	lines := reader.Lines("./data/day11/input.txt")
	stones := make(map[int]int)
	nums := pie.Ints(strings.Split(lines[0], " "))
	for _, n := range nums {
		_, ok := stones[n]
		if !ok {
			stones[n] = 0
		}
		stones[n] += 1
	}
	return stones
}

func blink(stones map[int]int, times int) map[int]int {
	for n := 0; n < times; n++ {
		new_stones := make(map[int]int)
		for val, num := range stones {
			s := strconv.Itoa(val)
			switch {
			case val == 0:
				_, ok_zero := new_stones[1]
				if !ok_zero {
					new_stones[1] = 0
				}
				new_stones[1] += num
			case len(s)%2 == 0:
				idx := len(s) / 2
				left := s[:idx]
				right := s[idx:]
				i1, _ := strconv.Atoi(left)
				i2, _ := strconv.Atoi(right)
				_, ok_left := new_stones[i1]
				if !ok_left {
					new_stones[i1] = 0
				}
				new_stones[i1] += num
				_, ok_right := new_stones[i2]
				if !ok_right {
					new_stones[i2] = 0
				}
				new_stones[i2] += num
			default:
				dval := val * 2024
				_, ok_def := new_stones[dval]
				if !ok_def {
					new_stones[dval] = 0
				}
				new_stones[dval] += num
			}
		}
		stones = new_stones
	}
	return stones
}
