package day24

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 24)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	device := data()
	return device.Checksum()
}

func Puzzle2() string {
	device := data()
	device = device.SwapOutputs("fgc", "z12")
	device = device.SwapOutputs("mtj", "z29")
	device = device.SwapOutputs("dgr", "vvm")
	device = device.SwapOutputs("dtv", "z37")
	wires := []string{"fgc", "z12", "mtj", "z29", "dgr", "vvm", "dtv", "z37"}
	sort.Strings(wires)
	return strings.Join(wires, ",")
}

// ========== PRIVATE FNS =================================

func data() Device {
	wires := make(map[string]int)
	gates := make(map[string]Gate)

	lines := reader.Lines("./data/day24/input.txt")
	for _, line := range lines {
		if strings.Index(line, ":") >= 0 {
			parts := strings.Split(line, ": ")
			id := parts[0]
			val, _ := strconv.Atoi(parts[1])
			wires[id] = val
		}
		if strings.Index(line, "->") >= 0 {
			re := regexp.MustCompile("^(\\S+) (\\S+) (\\S+) -> (\\S+)$")
			m := re.FindAllStringSubmatch(line, 1)[0]
			gate := Gate{inputs: []string{m[1], m[3]}, condition: m[2], output: m[4]}
			gates[gate.output] = gate
		}
	}

	return Device{wires: wires, gates: gates}
}
