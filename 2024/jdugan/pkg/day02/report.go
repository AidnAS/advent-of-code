package day02

import (
	"aoc/2024/pkg/utility"
	"slices"
	"sort"
)

// ========== DEFINITION ==================================

type Report struct {
	values []int
}

// ========== RECEIVERS ===================================

func (r Report) IsSafe() bool {
	safe := sort.IntsAreSorted(r.values)
	if !safe {
		slices.Reverse(r.values)
		safe = sort.IntsAreSorted(r.values)
	}
	if safe {
		prev := r.values[0]
		for _, curr := range r.values[1:] {
			if curr <= prev || utility.Distance(prev, curr) > 3 {
				safe = false
				break
			}
			prev = curr
		}
	}
	return safe
}

func (r Report) IsSortaSafe() bool {
	safe := false
	for i := 0; i < len(r.values); i++ {
		values := make([]int, len(r.values))
		copy(values, r.values)
		vals := append(values[:i], values[i+1:]...)
		report := Report{values: vals}
		if report.IsSafe() {
			safe = true
			break
		}
	}
	return safe
}

// ---------- UTILITIES -----------------------------------
