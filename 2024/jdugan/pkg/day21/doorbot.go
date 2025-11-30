package day21

import (
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type DoorBot struct{}

// ========== PUBLIC ======================================

func (b DoorBot) InstructionsFor(code string) string {
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

// ---------- PRIVATE -------------------------------------

func (b DoorBot) Routes() map[string]string {
	routes := make(map[string]string)

	// only one good route
	routes["A,0"] = "<A"
	routes["A,1"] = "^<<A"
	routes["A,3"] = "^A"
	routes["A,4"] = "^^<<A"
	routes["A,9"] = "^^^A"
	routes["0,A"] = ">A"
	routes["0,2"] = "^A"
	routes["0,8"] = "^^^A"
	routes["1,7"] = "^^A"
	routes["2,8"] = "^^A"
	routes["3,A"] = "vA"
	routes["4,5"] = ">A"
	routes["4,6"] = ">>A"
	routes["5,0"] = "vvA"
	routes["5,6"] = ">A"
	routes["6,A"] = "vvA"
	routes["6,3"] = "vA"
	routes["7,9"] = ">>A"
	routes["8,0"] = "vvvA"
	routes["9,A"] = "vvvA"
	routes["9,8"] = "<A"

	// two possible routes, found best through
	// trial and error.
	routes["A,2"] = "<^A"   // better than "^<A"
	routes["A,5"] = "<^^A"  // better than "^^<A"
	routes["2,4"] = "<^A"   // better than "^<A"
	routes["2,7"] = "<^^A"  // better than "^^<A"
	routes["3,7"] = "<<^^A" // better than "^^<<A"
	routes["8,A"] = "vvv>A" // better than ">vvvA"
	routes["8,6"] = "v>A"   // better than ">vA"

	// two possible routes, doesn't matter
	routes["2,9"] = ">^^A" // same as "^^>A"

	return routes
}
