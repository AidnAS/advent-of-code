package day24

import (
	"fmt"
	"slices"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Gate struct {
	inputs    []string
	condition string
	output    string
}

// ========== RECEIVERS ===================================

func (g Gate) Evaluate(wires map[string]int) (string, int) {
	id := g.output
	val := -1
	i1, ok1 := wires[g.inputs[0]]
	i2, ok2 := wires[g.inputs[1]]
	if ok1 && ok2 {
		switch {
		case g.condition == "AND" && (i1 == 1 && i2 == 1):
			val = 1
		case g.condition == "OR" && (i1 == 1 || i2 == 1):
			val = 1
		case g.condition == "XOR" && (i1 == 1 && i2 == 0):
			val = 1
		case g.condition == "XOR" && (i1 == 0 && i2 == 1):
			val = 1
		default:
			val = 0
		}
	}
	return id, val
}

func (g Gate) Print() {
	prefix := "   "
	if len(g.inputs) > 0 {
		if slices.Contains([]string{"x", "y"}, string(g.inputs[0][0])) {
			prefix = " "
		}
	}
	fmt.Println(prefix, pie.First(g.inputs), g.condition, pie.Last(g.inputs), "->", g.output)
}
