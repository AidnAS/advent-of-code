package day04

import (
	"aoc/2024/pkg/utility"
	"fmt"
	"strings"
)

// ========== DEFINITION ==================================

type Grid struct {
	points map[string]Point
}

// ========== RECEIVERS ===================================

func (g Grid) Find(point Point, patterns [][][]int, word string) [][]string {
	matches := make([][]string, 0)
	for _, pattern := range patterns {
		p := point
		ids := []string{p.Id()}
		vals := []string{p.value}
		for _, step := range pattern {
			new_id := utility.CoordToId(point.x+step[0], point.y+step[1])
			np, ok := g.points[new_id]
			new_val := "."
			if ok {
				new_val = np.value
			}
			ids = append(ids, new_id)
			vals = append(vals, new_val)
		}
		if strings.Join(vals, "") == word {
			matches = append(matches, ids)
		}
	}
	return matches
}

func (g Grid) Search(patterns [][][]int, word string) [][]string {
	x0, x1, y0, y1 := g.Dimensions()
	wstart := string(word[0])

	matches := make([][]string, 0)
	for x := x0; x <= x1; x++ {
		for y := y0; y <= y1; y++ {
			id := utility.CoordToId(x, y)
			p := g.points[id]
			if p.value == wstart {
				submatches := g.Find(p, patterns, word)
				matches = append(matches, submatches...)
			}
		}
	}
	return matches
}

// ---------- UTILITIES -----------------------------------

func (g Grid) Dimensions() (int, int, int, int) {
	min_x := 999999999
	max_x := 0
	min_y := 999999999
	max_y := 0
	for _, p := range g.points {
		switch {
		case p.x < min_x:
			min_x = p.x
		case p.x > max_x:
			max_x = p.x
		case p.y < min_y:
			min_y = p.y
		case p.y > max_y:
			max_y = p.y
		}
	}
	return min_x, max_x, min_y, max_y
}

func (g Grid) Print(dvalue string) {
	min_x, max_x, min_y, max_y := g.Dimensions()

	fmt.Println("")
	for y := min_y; y <= max_y; y++ {
		row := ""
		for x := min_x; x <= max_x; x++ {
			id := utility.CoordToId(x, y)
			p, ok := g.points[id]
			if ok {
				row += p.value
			} else {
				row += dvalue
			}
		}
		fmt.Println(row)
	}
	fmt.Println("")
}
