package aoc

import (
	"testing"

	. "github.com/franela/goblin"

	"aoc/2024/pkg/day01"
	"aoc/2024/pkg/day02"
	"aoc/2024/pkg/day03"
	"aoc/2024/pkg/day04"
	"aoc/2024/pkg/day05"
	"aoc/2024/pkg/day06"
	"aoc/2024/pkg/day07"
	"aoc/2024/pkg/day08"
	"aoc/2024/pkg/day09"
	"aoc/2024/pkg/day10"
	"aoc/2024/pkg/day11"
	"aoc/2024/pkg/day12"
	"aoc/2024/pkg/day13"
	"aoc/2024/pkg/day14"
	"aoc/2024/pkg/day15"
	"aoc/2024/pkg/day16"
	"aoc/2024/pkg/day17"
	"aoc/2024/pkg/day18"
	"aoc/2024/pkg/day19"
	"aoc/2024/pkg/day20"
	"aoc/2024/pkg/day21"
	"aoc/2024/pkg/day22"
	"aoc/2024/pkg/day23"
	"aoc/2024/pkg/day24"
	"aoc/2024/pkg/day25"
)

func Test(t *testing.T) {
	g := Goblin(t)

	g.Describe("AOC", func() {
		g.It("Should get correct answers for Day 01", func() {
			g.Assert(day01.Puzzle1()).Equal(2375403)
			g.Assert(day01.Puzzle2()).Equal(23082277)
		})
		g.It("Should get correct answers for Day 02", func() {
			g.Assert(day02.Puzzle1()).Equal(572)
			g.Assert(day02.Puzzle2()).Equal(612)
		})
		g.It("Should get correct answers for Day 03", func() {
			g.Assert(day03.Puzzle1()).Equal(180233229)
			g.Assert(day03.Puzzle2()).Equal(95411583)
		})
		g.It("Should get correct answers for Day 04", func() {
			g.Assert(day04.Puzzle1()).Equal(2639)
			g.Assert(day04.Puzzle2()).Equal(2005)
		})
		g.It("Should get correct answers for Day 05", func() {
			g.Assert(day05.Puzzle1()).Equal(6260)
			g.Assert(day05.Puzzle2()).Equal(5346)
		})
		g.It("Should get correct answers for Day 06", func() {
			g.Assert(day06.Puzzle1()).Equal(5177)
			// g.Assert(day06.Puzzle2()).Equal(1686)   TOO SLOW!!
		})
		g.It("Should get correct answers for Day 07", func() {
			g.Assert(day07.Puzzle1()).Equal(1399219271639)
			g.Assert(day07.Puzzle2()).Equal(275791737999003)
		})
		g.It("Should get correct answers for Day 08", func() {
			g.Assert(day08.Puzzle1()).Equal(413)
			g.Assert(day08.Puzzle2()).Equal(1417)
		})
		g.It("Should get correct answers for Day 09", func() {
			g.Assert(day09.Puzzle1()).Equal(6415184586041)
			g.Assert(day09.Puzzle2()).Equal(6436819084274)
		})
		g.It("Should get correct answers for Day 10", func() {
			g.Assert(day10.Puzzle1()).Equal(-1)
			g.Assert(day10.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 11", func() {
			g.Assert(day11.Puzzle1()).Equal(-1)
			g.Assert(day11.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 12", func() {
			g.Assert(day12.Puzzle1()).Equal(-1)
			g.Assert(day12.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 13", func() {
			g.Assert(day13.Puzzle1()).Equal(-1)
			g.Assert(day13.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 14", func() {
			g.Assert(day14.Puzzle1()).Equal(-1)
			g.Assert(day14.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 15", func() {
			g.Assert(day15.Puzzle1()).Equal(-1)
			g.Assert(day15.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 16", func() {
			g.Assert(day16.Puzzle1()).Equal(-1)
			g.Assert(day16.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 17", func() {
			g.Assert(day17.Puzzle1()).Equal(-1)
			g.Assert(day17.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 18", func() {
			g.Assert(day18.Puzzle1()).Equal(-1)
			g.Assert(day18.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 19", func() {
			g.Assert(day19.Puzzle1()).Equal(-1)
			g.Assert(day19.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 20", func() {
			g.Assert(day20.Puzzle1()).Equal(-1)
			g.Assert(day20.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 21", func() {
			g.Assert(day21.Puzzle1()).Equal(-1)
			g.Assert(day21.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 22", func() {
			g.Assert(day22.Puzzle1()).Equal(-1)
			g.Assert(day22.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 23", func() {
			g.Assert(day23.Puzzle1()).Equal(-1)
			g.Assert(day23.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 24", func() {
			g.Assert(day24.Puzzle1()).Equal(-1)
			g.Assert(day24.Puzzle2()).Equal(-2)
		})
		g.It("Should get correct answers for Day 25", func() {
			g.Assert(day25.Puzzle1()).Equal(-1)
			g.Assert(day25.Puzzle2()).Equal(50)
		})
	})
}
