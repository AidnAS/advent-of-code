package day21

import (
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type ControlBot struct{}

// ========== RECEIVERS ===================================

func (b ControlBot) SizeAfterIterations(code string, times int) int {
	cache := make(map[string]string)
	segments := make(map[string]int)
	segments[code] = 1
	for t := 1; t <= times; t++ {
		segments, cache = b.ProcessSegments(segments, cache)
	}
	return b.SegmentsSize(segments)
}

// ---------- PRIVATE -------------------------------------

func (b ControlBot) InstructionsFor(code string) string {
	routes := b.Routes()
	sb := strings.Builder{}
	head := "A"
	tail := strings.Split(code, "")
	for len(tail) > 0 {
		key := head + "," + tail[0]
		sb.WriteString(routes[key])
		head, tail = pie.Shift(tail)
	}
	return sb.String()
}

func (b ControlBot) ProcessSegments(old_segments map[string]int, cache map[string]string) (map[string]int, map[string]string) {
	segments := make(map[string]int)
	for code, count := range old_segments {
		new_code, found := cache[code]
		if !found {
			new_code = b.InstructionsFor(code)
		}
		parts := strings.Split(new_code, "A")
		parts = parts[:len(parts)-1]
		for _, k := range parts {
			k += "A"
			v := segments[k] // zero if key not found
			segments[k] = v + count
		}
	}
	return segments, cache
}

func (b ControlBot) Routes() map[string]string {
	routes := make(map[string]string)

	// only one good route
	routes["A,A"] = "A"
	routes["A,^"] = "<A"
	routes["A,<"] = "v<<A"
	routes["A,>"] = "vA"
	routes["^,A"] = ">A"
	routes["^,^"] = "A"
	routes["^,v"] = "vA"
	routes["^,<"] = "v<A"
	routes["v,^"] = "^A"
	routes["v,v"] = "A"
	routes["v,<"] = "<A"
	routes["v,>"] = ">A"
	routes["<,A"] = ">>^A"
	routes["<,^"] = ">^A"
	routes["<,v"] = ">A"
	routes["<,<"] = "A"
	routes["<,>"] = ">>A"
	routes[">,A"] = "^A"
	routes[">,v"] = "<A"
	routes[">,<"] = "<<A"
	routes[">,>"] = "A"

	// two possible routes, found best through
	// trial and error. :)
	routes["A,v"] = "<vA" // better than "v<A"
	routes["^,>"] = "v>A" // better than ">vA"
	routes["v,A"] = "^>A" // better than ">^A"
	routes[">,^"] = "<^A" // better than "^<A"

	return routes
}

func (b ControlBot) SegmentsSize(segments map[string]int) int {
	sum := 0
	for code, count := range segments {
		sum += len(code) * count
	}
	return sum
}
