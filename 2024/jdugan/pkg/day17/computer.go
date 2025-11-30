package day17

import (
	"math"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Computer struct {
	memory  map[string]int
	output  []int
	pointer int
}

// ========== RECEIVERS ===================================

func (c *Computer) Run(program []int) {
	for c.pointer < len(program) {
		opscode := program[c.pointer]
		operand := program[c.pointer+1]
		c.pointer += 2

		switch opscode {
		// adv
		case 0:
			combo, _ := c.ComboValue(operand)
			denom := int(math.Pow(2, float64(combo)))
			c.memory["A"] = c.memory["A"] / denom
		//bxl
		case 1:
			c.memory["B"] = c.memory["B"] ^ operand
		// bst
		case 2:
			combo, _ := c.ComboValue(operand)
			c.memory["B"] = combo % 8
		// jnz
		case 3:
			if c.memory["A"] != 0 {
				c.pointer = operand
			}
		// bxc
		case 4:
			c.memory["B"] = c.memory["B"] ^ c.memory["C"]
		// out
		case 5:
			combo, _ := c.ComboValue(operand)
			c.output = append(c.output, combo%8)
		// bdv
		case 6:
			combo, _ := c.ComboValue(operand)
			denom := int(math.Pow(2, float64(combo)))
			c.memory["B"] = c.memory["A"] / denom
		// cdv
		case 7:
			combo, _ := c.ComboValue(operand)
			denom := int(math.Pow(2, float64(combo)))
			c.memory["C"] = c.memory["A"] / denom
		}
	}
}

func (c *Computer) Search(program []int) int {
	idx := len(program) - 1
	seed := int(math.Pow(8, float64(idx)))
	c.Reset(seed)
	c.Run(program)
	for {
		power := pie.Max([]int{0, idx - 1})
		seed += int(math.Pow(8, float64(power)))
		c.Reset(seed)
		c.Run(program)
		omatch := strings.Join(pie.Strings(c.output[idx:]), ",")
		tmatch := strings.Join(pie.Strings(program[idx:]), ",")
		if omatch == tmatch {
			if idx == 0 {
				break
			} else {
				idx -= 1
			}
		}
	}

	return seed
}

// ---------- UTILITIES -----------------------------------

func (c Computer) ComboValue(operand int) (int, bool) {
	switch operand {
	case 0, 1, 2, 3:
		return operand, true
	case 4:
		return c.memory["A"], true
	case 5:
		return c.memory["B"], true
	case 6:
		return c.memory["C"], true
	default:
		return -1, false
	}
}

func (c *Computer) Reset(a int) {
	c.memory["A"] = a
	c.memory["B"] = 0
	c.memory["C"] = 0
	c.pointer = 0
	c.output = make([]int, 0)
}

func (c Computer) Result() string {
	return strings.Join(pie.Strings(c.output), ",")
}
