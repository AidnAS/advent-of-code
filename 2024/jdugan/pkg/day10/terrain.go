package day10

import (
	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Terrain struct {
	points map[string]Point
}

// ========== RECEIVERS ===================================

func (t Terrain) GetTrailSystemRating() int {
	tmap := t.GetTrailSystem()
	sum := 0
	for _, trails := range tmap {
		sum += len(trails)
	}
	return sum
}

func (t Terrain) GetTrailSystemScore() int {
	tmap := t.GetTrailSystem()
	sum := 0
	for _, trails := range tmap {
		dests := make([]string, 0)
		for _, trail := range trails {
			dests = append(dests, pie.Last(trail))
		}
		sum += len(pie.Unique(dests))
	}
	return sum
}

// ---------- HELPERS -------------------------------------

func (t Terrain) AdjacentPoints(p Point) []Point {
	aps := make([]Point, 0)
	for _, aid := range p.AdjacentIds() {
		p, ok := t.points[aid]
		if ok {
			aps = append(aps, p)
		}
	}
	return aps
}

func (t Terrain) GetTrailHeads() []Point {
	theads := make([]Point, 0)
	for _, p := range t.points {
		if p.value == 0 {
			theads = append(theads, p)
		}
	}
	return theads
}

func (t Terrain) GetTrailSystem() map[string][][]string {
	tmap := make(map[string][][]string)
	for _, thead := range t.GetTrailHeads() {
		trails := make([][]string, 0)
		possibles := [][]string{[]string{thead.Id()}}
		for len(possibles) > 0 {
			new_possibles := make([][]string, 0)
			for _, possible := range possibles {
				bpid := pie.Last(possible)
				bp, _ := t.points[bpid]
				aps := t.AdjacentPoints(bp)
				for _, ap := range aps {
					if ap.value == bp.value+1 {
						cp := make([]string, len(possible))
						copy(cp, possible)
						cp = append(cp, ap.Id())
						if ap.value == 9 {
							trails = append(trails, cp)
						} else {
							new_possibles = append(new_possibles, cp)
						}
					}
				}

			}
			possibles = new_possibles
		}
		tmap[thead.Id()] = trails
	}
	return tmap
}

func (t *Terrain) UpdatePoint(p Point) {
	t.points[p.Id()] = p
}
