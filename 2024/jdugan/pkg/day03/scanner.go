package day03

import (
	"regexp"
	"strconv"
	"strings"
)

// ========== DEFINITION ==================================

type Scanner struct {
	program string
}

// ========== RECEIVERS ===================================

func (s Scanner) ComplexParse() int {
	r, _ := regexp.Compile("mul\\(\\d+,\\d+\\)|do\\(\\)|don't\\(\\)")
	cmds := r.FindAllString(s.program, -1)
	enabled := true
	sum := 0
	for _, cmd := range cmds {
		switch cmd {
		case "do()":
			enabled = true
		case "don't()":
			enabled = false
		default:
			if enabled {
				sum += s.CommandValue(cmd)
			}
		}
	}
	return sum
}

func (s Scanner) SimpleParse() int {
	r, _ := regexp.Compile("mul\\(\\d+,\\d+\\)")
	cmds := r.FindAllString(s.program, -1)
	sum := 0
	for _, cmd := range cmds {
		sum += s.CommandValue(cmd)
	}
	return sum
}

// ---------- UTILITIES -----------------------------------

func (s Scanner) CommandValue(cmd string) int {
	parts := strings.Split(cmd, ",")
	s1 := strings.Replace(parts[0], "mul(", "", 1)
	s2 := strings.Replace(parts[1], ")", "", 1)
	i1, _ := strconv.Atoi(s1)
	i2, _ := strconv.Atoi(s2)
	return i1 * i2
}
