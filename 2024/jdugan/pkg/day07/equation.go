package day07

import (
	"strconv"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Equation struct {
	result  int
	factors []int
}

// ========== RECEIVERS ===================================

func (e Equation) FindSolutions(operands []string) []Solution {
	head, tail := pie.Shift(e.factors)
	solutions := []Solution{Solution{result: head}}
	for len(tail) > 0 {
		head, tail = pie.Shift(tail)
		possibles := make([]Solution, 0)
		for _, s := range solutions {
			for _, operand := range operands {
				switch operand {
				case "+":
					res := s.result + head
					if res <= e.result {
						ops := append(s.operations, "+")
						possibles = append(possibles, Solution{result: res, operations: ops})
					}
				case "*":
					res := s.result * head
					if res <= e.result {
						ops := append(s.operations, "*")
						possibles = append(possibles, Solution{result: res, operations: ops})
					}
				case "||":
					res, _ := strconv.Atoi(strconv.Itoa(s.result) + strconv.Itoa(head))
					if res <= e.result {
						ops := append(s.operations, "*")
						possibles = append(possibles, Solution{result: res, operations: ops})
					}
				}
			}
		}
		solutions = possibles
	}
	valids := pie.Filter(solutions, func(s Solution) bool { return s.result == e.result })
	return valids
}
