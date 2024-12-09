package day06

import (
	"aoc/2024/pkg/utility"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Guard struct {
	current State
	path    []State
}

// ========== RECEIVERS ===================================

func (g *Guard) PatrolMap(points map[string]Point) map[string][]string {
	pmap := make(map[string][]string, 0)
	path, _ := g.Patrol(points)
	for _, state := range path {
		dirs, ok := pmap[state.position]
		if ok {
			pmap[state.position] = append(dirs, state.direction)
		} else {
			pmap[state.position] = []string{state.direction}
		}
	}
	return pmap
}

func (g *Guard) Patrol(points map[string]Point) ([]State, bool) {
	looped := false
	for {
		fid := g.forwardId()
		fp, ok := points[fid]
		if ok {
			if fp.value == "#" {
				g.Turn()
			} else {
				matches := pie.Filter(g.path, func(s State) bool {
					return s.position == fid && s.direction == g.current.direction
				})
				if len(matches) > 0 {
					looped = true
					break
				} else {
					g.path = append(g.path, g.current)
					g.current = State{position: fp.Id(), direction: g.current.direction}
				}
			}
		} else {
			g.path = append(g.path, g.current)
			break
		}
	}
	return g.path, looped
}

func (g *Guard) Turn() {
	dir := g.current.direction
	switch dir {
	case "E":
		dir = "S"
	case "N":
		dir = "E"
	case "S":
		dir = "W"
	case "W":
		dir = "N"
	}
	g.path = append(g.path, g.current)
	g.current = State{position: g.current.position, direction: dir}
}

// ---------- UTILITIES -----------------------------------

func (g Guard) forwardId() string {
	id := g.current.position
	x, y := utility.CoordFromId(id)
	p := Point{x: x, y: y}
	switch g.current.direction {
	case "E":
		id = p.EastId()
	case "N":
		id = p.NorthId()
	case "S":
		id = p.SouthId()
	default:
		id = p.WestId()
	}
	return id
}
