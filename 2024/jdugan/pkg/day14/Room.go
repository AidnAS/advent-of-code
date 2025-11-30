package day14

import (
	"aoc/2024/pkg/utility"
	"fmt"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Room struct {
	width  int
	height int
	robots []Robot
}

// ========== RECEIVERS ===================================

func (r Room) FirstChristmasTree() int {
	best_score := 0
	best_tick := 0
	ticks := 0
	for ticks < r.width*r.height {
		ticks += 1
		r.ElapseTime(1)
		score := r.AdjacencyScore()
		if score > best_score {
			best_score = score
			best_tick = ticks
		}
	}
	return best_tick
}

func (r Room) SafetyFactor() int {
	nw, ne, sw, se := r.QuadrantCounts()
	return nw * ne * sw * se
}

// ---------- UTILITIES -----------------------------------

func (r Room) AdjacencyScore() int {
	rmap := make(map[string]Robot)
	for _, rb := range r.robots {
		rmap[rb.Id()] = rb
	}
	amap := make(map[int]int)
	for i := 0; i < 5; i++ {
		amap[i] = 0
	}
	for _, rb := range r.robots {
		count := 0
		for _, id := range rb.AdjacentIds() {
			_, ok := rmap[id]
			if ok {
				count += 1
			}
		}
		amap[count] += count
	}
	score := 0
	for k, v := range amap {
		score += k * v
	}
	return score
}

func (r Room) AdjustDimension(pos int, delta int, times int, max int) int {
	pos = pos + (delta * times)
	pos = pos % max
	pos = pos + max
	return pos % max
}

func (r *Room) ElapseTime(ticks int) {
	robots := make([]Robot, 0)
	for _, rb := range r.robots {
		rb.x = r.AdjustDimension(rb.x, rb.dx, ticks, r.width)
		rb.y = r.AdjustDimension(rb.y, rb.dy, ticks, r.height)
		robots = append(robots, rb)
	}
	r.robots = robots
}

func (r Room) IsSymmetrical() bool {
	qmap := r.QuadrantIds()
	symmetrical := false
	// unique counts equal over veritcal midline
	if symmetrical {
		nw_len := len(pie.Unique(qmap["nw"]))
		ne_len := len(pie.Unique(qmap["ne"]))
		sw_len := len(pie.Unique(qmap["sw"]))
		se_len := len(pie.Unique(qmap["se"]))
		if nw_len == ne_len && sw_len == se_len {
			symmetrical = true
		}
	}
	return symmetrical
}

func (r Room) Print() {
	rmap := make(map[string]Robot)
	for _, rb := range r.robots {
		id := utility.CoordToId(rb.x, rb.y)
		rmap[id] = rb
	}
	for y := 0; y < r.height; y++ {
		row := ""
		for x := 0; x < r.width; x++ {
			id := utility.CoordToId(x, y)
			_, ok := rmap[id]
			if ok {
				row += "#"
			} else {
				row += "."
			}
		}
		fmt.Println(row)
	}
	fmt.Println("")
	fmt.Println("")
	fmt.Println("")
}

func (r Room) QuadrantIds() map[string][]string {
	bx := r.width / 2
	by := r.height / 2

	qmap := make(map[string][]string)
	qmap["nw"] = make([]string, 0)
	qmap["ne"] = make([]string, 0)
	qmap["sw"] = make([]string, 0)
	qmap["se"] = make([]string, 0)

	for _, rb := range r.robots {
		id := utility.CoordToId(rb.x, rb.y)
		switch {
		case rb.x < bx && rb.y < by:
			qmap["nw"] = append(qmap["nw"], id)
		case rb.x > bx && rb.y < by:
			qmap["ne"] = append(qmap["ne"], id)
		case rb.x < bx && rb.y > by:
			qmap["sw"] = append(qmap["sw"], id)
		case rb.x > bx && rb.y > by:
			qmap["se"] = append(qmap["se"], id)
		}
	}
	return qmap
}

func (r Room) QuadrantCounts() (int, int, int, int) {
	qmap := r.QuadrantIds()
	nw := len(qmap["nw"])
	ne := len(qmap["ne"])
	sw := len(qmap["sw"])
	se := len(qmap["se"])
	return nw, ne, sw, se
}
