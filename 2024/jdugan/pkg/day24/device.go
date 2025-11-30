package day24

import (
	"fmt"
	"slices"
	"sort"
	"strconv"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Device struct {
	wires map[string]int
	gates map[string]Gate
}

// ========== RECEIVERS ===================================

func (d Device) Checksum() int {
	wires := d.CompleteCircuit()
	return d.DecimalWireGroupValue(wires, "z")
}

func (d Device) SwapOutputs(o1 string, o2 string) Device {
	g1 := d.gates[o1]
	g2 := d.gates[o2]
	ng1 := Gate{inputs: g2.inputs, condition: g2.condition, output: o1}
	ng2 := Gate{inputs: g1.inputs, condition: g1.condition, output: o2}
	gates := d.gates
	gates[o1] = ng1
	gates[o2] = ng2
	return Device{wires: d.wires, gates: gates}
}

// ---------- PRIVATE -------------------------------------

func (d Device) BinaryWireGroupValue(wires map[string]int, group string) string {
	ids := make([]string, 0)
	vals := make([]string, 0)
	for id := range wires {
		if string(id[0]) == group {
			ids = append(ids, id)
		}
	}
	sort.Strings(ids)
	slices.Reverse(ids)
	for _, id := range ids {
		vals = append(vals, strconv.Itoa(wires[id]))
	}
	return strings.Join(vals, "")
}

func (d Device) CompleteCircuit() map[string]int {
	wires := d.wires
	gates := pie.Values(d.gates)
	for len(gates) > 0 {
		new_gates := make([]Gate, 0)
		for _, g := range gates {
			id, val := g.Evaluate(wires)
			if val != -1 {
				wires[id] = val
			} else {
				new_gates = append(new_gates, g)
			}
		}
		gates = new_gates
	}
	return wires
}

func (d Device) DecimalWireGroupValue(wires map[string]int, group string) int {
	binary := d.BinaryWireGroupValue(wires, group)
	decimal, _ := strconv.ParseInt(binary, 2, 64)
	return int(decimal)
}

// --------------------------------------------------------
// FORMAT
// g1: x AND y -> w1
// g2: w1 OR w2 -> w0
// g3: x XOR y -> w3
// g4: w3 AND w4 -> w2
// g5: w3 XOR w4 -> z
//
// RULES
// -> slices.Contains(g2.inputs, g1.output)
// -> slices.Contains(g2.inputs, g4.output)
// -> slices.Contains(g4.inputs, g3.output)
// -> slices.Contains(g4.inputs, g5.inputs[0]) && slices.Contains(g4.inputs, g5.inputs[1])
// -> string(g5.output[0]) == "z"
// --------------------------------------------------------
func (d Device) Investigate() {
	gates := pie.Values(d.gates)
	bx := d.BinaryWireGroupValue(d.wires, "x")
	for idx := 1; idx < len(bx)-1; idx++ {
		xkey := d.WireKey("x", idx)
		mains := pie.Filter(gates, func(g Gate) bool { return slices.Contains(g.inputs, xkey) })
		g1 := pie.First(pie.Filter(mains, func(g Gate) bool { return g.condition == "AND" }))
		g2 := pie.First(pie.Filter(gates, func(g Gate) bool { return slices.Contains(g.inputs, g1.output) }))
		g3 := pie.First(pie.Filter(mains, func(g Gate) bool { return g.condition == "XOR" }))
		subs := pie.Filter(gates, func(g Gate) bool { return slices.Contains(g.inputs, g3.output) })
		g4 := pie.First(pie.Filter(subs, func(g Gate) bool { return g.condition == "AND" }))
		g5 := pie.First(pie.Filter(subs, func(g Gate) bool { return g.condition == "XOR" }))
		error := false
		if !slices.Contains(g2.inputs, g1.output) ||
			!slices.Contains(g2.inputs, g4.output) ||
			!slices.Contains(g4.inputs, g3.output) ||
			!slices.Contains(g4.inputs, pie.First(g5.inputs)) ||
			!slices.Contains(g4.inputs, pie.Last(g5.inputs)) ||
			string(g5.output[0]) != "z" {
			error = true
		}
		if error {
			fmt.Println(idx)
			g1.Print()
			g2.Print()
			g3.Print()
			g4.Print()
			g5.Print()
			fmt.Println("")
		}
	}
	fmt.Println(d.IsFunctional())
}

func (d Device) IsFunctional() bool {
	x := d.DecimalWireGroupValue(d.wires, "x")
	y := d.DecimalWireGroupValue(d.wires, "y")
	wires := d.CompleteCircuit()
	z := d.DecimalWireGroupValue(wires, "z")
	return x+y == z
}

func (d Device) WireKey(group string, index int) string {
	snum := strconv.Itoa(index)
	if len(snum) == 1 {
		snum = "0" + snum
	}
	return group + snum
}
