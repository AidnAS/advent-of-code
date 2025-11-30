package day21

import (
	"aoc/2024/pkg/reader"
	"fmt"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 21)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	cmds, dbot, cbot := data()
	sum := 0
	for _, cmd := range cmds {
		code := dbot.InstructionsFor(cmd.code)
		size := cbot.SizeAfterIterations(code, 2)
		sum += cmd.Multiplier() * size
	}
	return sum
}

// 157055032722640
func Puzzle2() int {
	cmds, dbot, cbot := data()
	sum := 0
	for _, cmd := range cmds {
		code := dbot.InstructionsFor(cmd.code)
		size := cbot.SizeAfterIterations(code, 25)
		sum += cmd.Multiplier() * size
	}
	return sum
}

// ========== PRIVATE FNS =================================

func data() ([]Command, DoorBot, ControlBot) {
	lines := reader.Lines("./data/day21/input.txt")
	cmds := make([]Command, 0)
	for _, line := range lines {
		cmds = append(cmds, Command{code: line})
	}
	dbot := DoorBot{}
	cbot := ControlBot{}
	return cmds, dbot, cbot
}
