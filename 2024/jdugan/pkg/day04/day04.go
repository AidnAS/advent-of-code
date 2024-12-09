package day04

import (
	"aoc/2024/pkg/reader"
	"fmt"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 4)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	grid := data()
	matches := grid.Search(linearPatterns(), "XMAS")
	return len(matches)
}

func Puzzle2() int {
	grid := data()
	matches := grid.Search(crossPatterns(), "MMASS")
	return len(matches)
}

// ========== PRIVATE FNS =================================

func data() Grid {
	lines := reader.Lines("./data/day04/input.txt")

	points := make(map[string]Point)
	for y, row := range lines {
		for x, col := range row {
			p := Point{x: x, y: y, value: string(col)}
			points[p.Id()] = p
		}
	}
	return Grid{points: points}
}

func crossPatterns() [][][]int {
	patterns := make([][][]int, 0)
	patterns = append(patterns, [][]int{[]int{2, 0}, []int{1, -1}, []int{0, -2}, []int{2, -2}})    // e
	patterns = append(patterns, [][]int{[]int{0, -2}, []int{-1, -1}, []int{-2, 0}, []int{-2, -2}}) // n
	patterns = append(patterns, [][]int{[]int{-2, 0}, []int{-1, 1}, []int{0, 2}, []int{-2, 2}})    // w
	patterns = append(patterns, [][]int{[]int{0, 2}, []int{1, 1}, []int{2, 0}, []int{2, 2}})       // s
	return patterns
}
func linearPatterns() [][][]int {
	patterns := make([][][]int, 0)
	patterns = append(patterns, [][]int{[]int{1, 0}, []int{2, 0}, []int{3, 0}})       // e
	patterns = append(patterns, [][]int{[]int{0, -1}, []int{0, -2}, []int{0, -3}})    // n
	patterns = append(patterns, [][]int{[]int{1, -1}, []int{2, -2}, []int{3, -3}})    // ne
	patterns = append(patterns, [][]int{[]int{-1, -1}, []int{-2, -2}, []int{-3, -3}}) // nw
	patterns = append(patterns, [][]int{[]int{0, 1}, []int{0, 2}, []int{0, 3}})       // s
	patterns = append(patterns, [][]int{[]int{1, 1}, []int{2, 2}, []int{3, 3}})       // se
	patterns = append(patterns, [][]int{[]int{-1, 1}, []int{-2, 2}, []int{-3, 3}})    // sw
	patterns = append(patterns, [][]int{[]int{-1, 0}, []int{-2, 0}, []int{-3, 0}})    // w
	return patterns
}
