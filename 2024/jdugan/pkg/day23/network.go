package day23

import (
	"slices"
	"sort"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Network struct {
	graph map[string][]string
}

// ========== RECEIVERS ===================================

func (n Network) Checksum() int {
	groups := n.BaseGroups()        // groups of 1
	groups = n.ExpandGroups(groups) // groups of 2
	groups = n.ExpandGroups(groups) // groups of 3
	groups = n.WithTeeNodes(groups)
	return len(groups)
}

func (n Network) FindLargestGroup() []string {
	groups := n.BaseGroups()
	for len(groups) > 1 {
		groups = n.ExpandGroups(groups)
	}
	return groups[0]
}

// ---------- UTILITIES -----------------------------------

func (n Network) BaseGroups() [][]string {
	groups := make([][]string, 0)
	for n, _ := range n.graph {
		group := []string{n}
		groups = append(groups, group)
	}
	return groups
}

func (n Network) DedupGroups(groups [][]string) [][]string {
	new_groups := make([][]string, 0)
	ids := make([]string, 0)
	for _, group := range groups {
		id := strings.Join(group, ",")
		if !slices.Contains(ids, id) {
			new_groups = append(new_groups, group)
			ids = append(ids, id)
		}
	}
	return new_groups
}

func (n Network) ExpandGroups(groups [][]string) [][]string {
	new_groups := make([][]string, 0)
	for _, group := range groups {
		for node, connections := range n.graph {
			matches := pie.Intersect(group, connections)
			if len(matches) == len(group) {
				ngroup := append(group, node)
				sort.Strings(ngroup)
				new_groups = append(new_groups, ngroup)
			}
		}
	}
	return n.DedupGroups(new_groups)
}

func (n Network) WithTeeNodes(groups [][]string) [][]string {
	new_groups := make([][]string, 0)
	for _, group := range groups {
		found := false
		for _, node := range group {
			if string(node[0]) == "t" {
				found = true
				break
			}
		}
		if found {
			new_groups = append(new_groups, group)
		}
	}
	return new_groups
}
