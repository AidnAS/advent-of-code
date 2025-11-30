package day22

import (
	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Market struct {
	monkeys []Monkey
}

// ========== RECEIVERS ===================================

func (m Market) BestDeal() int {
	// build working sets
	keys := make([]string, 0)
	fmaps := make([]map[string]int, 0)
	for _, monkey := range m.monkeys {
		ks, fmap := monkey.FluctuationMap(1999)
		keys = pie.Unique(append(keys, ks...))
		fmaps = append(fmaps, fmap)
	}
	// determine best amount
	best := 0
	for _, k := range keys {
		sum := 0
		for _, fmap := range fmaps {
			sum += fmap[k] // not found defaults to zero
		}
		if sum > best {
			best = sum
		}
	}
	return best
}

func (m Market) Checksum() int {
	sum := 0
	for _, monkey := range m.monkeys {
		ps := monkey.Secrets(2000)
		sum += ps[1999]
	}
	return sum
}

// ---------- UTILITIES -----------------------------------
