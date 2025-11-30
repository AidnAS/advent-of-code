package day22

import (
	"slices"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Monkey struct {
	secret int
}

// ========== RECEIVERS ===================================

func (m Monkey) FluctuationMap(times int) ([]string, map[string]int) {
	keys := make([]string, 0)
	fmap := make(map[string]int)
	prices := make([]int, 0)
	deltas := make([]int, 0)
	head := m.secret
	tail := m.Secrets(times)
	for len(tail) > 0 {
		prices = append(prices, tail[0]%10)
		deltas = append(deltas, tail[0]%10-head%10)
		head, tail = pie.Shift(tail)
	}
	for idx := 0; idx < len(deltas)-4; idx++ {
		key := strings.Join(pie.Strings(deltas[idx:idx+4]), ",")
		if !slices.Contains(keys, key) {
			keys = append(keys, key)
			fmap[key] = prices[idx+3]
		}
	}
	return keys, fmap
}

func (m Monkey) Secrets(times int) []int {
	secrets := make([]int, 0)
	number := m.secret
	result := 0
	for i := 0; i < times; i++ {
		// step 1
		result = number * 64
		number = result ^ number
		number = number % 16777216
		// step 2
		result = number / 32
		number = result ^ number
		number = number % 16777216
		// step 3
		result = number * 2048
		number = result ^ number
		number = number % 16777216
		// add to list
		secrets = append(secrets, number)
	}
	return secrets
}
