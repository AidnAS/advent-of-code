package day23

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strings"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 23)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	network := data()
	return network.Checksum()
}

func Puzzle2() string {
	network := data()
	group := network.FindLargestGroup()
	return strings.Join(group, ",")
}

// ========== PRIVATE FNS =================================

func data() Network {
	lines := reader.Lines("./data/day23/input.txt")
	graph := make(map[string][]string)

	for _, line := range lines {
		names := strings.Split(line, "-")
		n1 := names[0]
		n2 := names[1]
		for _, n := range []string{n1, n2} {
			_, ok := graph[n]
			if !ok {
				graph[n] = make([]string, 0)
			}
		}
		graph[n1] = append(graph[n1], n2)
		graph[n2] = append(graph[n2], n1)
	}
	return Network{graph: graph}
}
