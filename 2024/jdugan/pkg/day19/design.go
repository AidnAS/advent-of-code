package day19

import (
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Design struct {
	pattern string
}

// ========== RECEIVERS ===================================

func (d Design) SequenceCount(tmap map[string][]string) int {
	sequences := 0
	smap := make(map[string]int)
	smap[d.pattern] = 1
	for len(smap) > 0 {
		new_smap := make(map[string]int)
		for pattern, pcount := range smap {
			tkey := string(pattern[0])
			matches := pie.Filter(tmap[tkey], func(t string) bool {
				return len(t) <= len(pattern) && t == pattern[:len(t)]
			})
			for _, match := range matches {
				skey := strings.Replace(pattern, match, "", 1)
				if skey == "" {
					sequences += pcount
				} else {
					scount, ok := new_smap[skey]
					if !ok {
						scount = 0
					}
					new_smap[skey] = scount + pcount
				}
			}
		}
		smap = new_smap
	}
	return sequences
}

func (d Design) IsPossible(tmap map[string][]string) bool {
	possible := false
	patterns := []string{d.pattern}
loop:
	for len(patterns) > 0 {
		new_patterns := make([]string, 0)
		for _, pattern := range patterns {
			k := string(pattern[0])
			matches := pie.Filter(tmap[k], func(t string) bool {
				return len(t) <= len(pattern) && t == pattern[:len(t)]
			})
			for _, match := range matches {
				np := strings.Replace(pattern, match, "", 1)
				if len(np) == 0 {
					possible = true
					break loop
				} else {
					new_patterns = append(new_patterns, np)
				}
			}
		}
		patterns = pie.Unique(new_patterns)
	}
	return possible
}

// ---------- UTILITIES -----------------------------------
