package day21

import (
	"strconv"
	"strings"
)

// ========== DEFINITION ==================================

type Command struct {
	code string
}

// ========== RECEIVERS ===================================

func (c Command) Multiplier() int {
	str := strings.Replace(c.code, "A", "", 1)
	num, _ := strconv.Atoi(str)
	return num
}
