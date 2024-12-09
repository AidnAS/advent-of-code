package day05

import (
	"slices"
)

// ========== DEFINITION ==================================

type PrintRun struct {
	pages    []int
	checksum int
}

// ========== RECEIVERS ===================================

func (pr PrintRun) IsValid(rules []Rule) (bool, int) {
	valid := true
	for _, r := range rules {
		bi := slices.Index(pr.pages, r.before)
		ai := slices.Index(pr.pages, r.after)
		if bi != -1 && ai != -1 && bi > ai {
			valid = false
			break
		}
	}
	return valid, pr.checksum
}

func (pr *PrintRun) Repair(rules []Rule) {
	valid := false
	for !valid {
		valid = true
		for _, r := range rules {
			bi := slices.Index(pr.pages, r.before)
			ai := slices.Index(pr.pages, r.after)
			if bi != -1 && ai != -1 && bi > ai {
				pr.pages[bi] = r.after
				pr.pages[ai] = r.before
				valid = false
			}
		}
	}
	pr.Reset()
}

func (pr *PrintRun) Reset() {
	idx := len(pr.pages) / 2
	pr.checksum = pr.pages[idx]
}
