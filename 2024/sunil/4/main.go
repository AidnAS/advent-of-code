package main

// Search for XMAS in input.txt. Word search allows words to be horizontal, vertical, diagonal,
// written backwards, or even overlapping other words.

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	// Open input file
	input, err := os.Open("input.txt")
	if err != nil {
		fmt.Println("Error opening file")
		return
	}
	defer input.Close()

	scanner := bufio.NewScanner(input)
	var grid map[string]rune = make(map[string]rune)
	var H, W int
	y := 0
	for scanner.Scan() {
		line := scanner.Text()
		W = len(line)
		for x, c := range line {
			grid[fmt.Sprintf("%d,%d", y, x)] = c
		}
		y++
	}
	H = y

	// fmt.Printf("grid: %v\n", grid)

	target := "XMAS"
	deltas := [][]int{{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1}}
	count := 0
	for y := 0; y < H; y++ {
		for x := 0; x < W; x++ {
			for _, delta := range deltas {
				dy, dx := delta[0], delta[1]
				candidate := ""
				for i := 0; i < len(target); i++ {
					cy, cx := y+dy*i, x+dx*i
					if cy < 0 || cy >= H || cx < 0 || cx >= W {
						break
					}
					candidate += string(grid[fmt.Sprintf("%d,%d", cy, cx)])
				}
				fmt.Printf("candidate: %s\n", candidate)
				if candidate == target {
					count++
				}
			}
		}
	}
	fmt.Println("Part 1:", count)

	count2 := 0
	for y := 0; y < H; y++ {
		for x := 0; x < W; x++ {
			if grid[fmt.Sprintf("%d,%d", y, x)] == 'A' {
				upper := string(grid[fmt.Sprintf("%d,%d", y-1, x-1)]) + string(grid[fmt.Sprintf("%d,%d", y+1, x+1)])
				lower := string(grid[fmt.Sprintf("%d,%d", y-1, x+1)]) + string(grid[fmt.Sprintf("%d,%d", y+1, x-1)])
				if (upper == "MS" || upper == "SM") && (lower == "MS" || lower == "SM") {
					count2++
				}
			}
		}
	}
	fmt.Println("Part 2:", count2)
}
