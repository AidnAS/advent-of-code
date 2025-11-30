package day15

import (
	"aoc/2024/pkg/utility"
	"fmt"

	"github.com/elliotchance/pie/v2"
)

// ========== DEFINITION ==================================

type Warehouse struct {
	moves  []string
	points map[string]Point
}

// ========== RECEIVERS ===================================

func (wh *Warehouse) Expand() {
	points := make(map[string]Point)
	for _, p := range wh.points {
		x1 := p.x * 2
		x2 := (p.x * 2) + 1
		y := p.y
		id1 := utility.CoordToId(x1, y)
		id2 := utility.CoordToId(x2, y)
		switch p.value {
		case "#":
			points[id1] = Point{x: x1, y: y, value: "#"}
			points[id2] = Point{x: x2, y: y, value: "#"}
		case ".":
			points[id1] = Point{x: x1, y: y, value: "."}
			points[id2] = Point{x: x2, y: y, value: "."}
		case "O":
			points[id1] = Point{x: x1, y: y, value: "["}
			points[id2] = Point{x: x2, y: y, value: "]"}
		case "@":
			points[id1] = Point{x: x1, y: y, value: "@"}
			points[id2] = Point{x: x2, y: y, value: "."}
		}
	}
	wh.points = points
}

func (wh Warehouse) GpsScore() int {
	score := 0
	for _, p := range wh.points {
		score += p.GpsScore()
	}
	return score
}

func (wh *Warehouse) PerformMoves() {
	robot := wh.GetRobot()
	for _, move := range wh.moves {
		oid := robot.Id()
		dx, dy := wh.ForwardOffsets(move)
		fps := wh.ForwardPoints(robot, move)
		if len(fps) > 0 {
			for _, fp := range fps {
				fp.x += dx
				fp.y += dy
				wh.points[fp.Id()] = fp
				if fp.IsRobot() {
					robot = fp
				}
			}
			// clear old robot position
			op := wh.points[oid]
			op.value = "."
			wh.points[oid] = op
			// clear big box orphans
			for id, _ := range wh.points {
				p, _ := wh.points[id]
				switch p.value {
				case "[":
					np, _ := wh.points[p.EastId()]
					if np.value != "]" {
						p.value = "."
						wh.points[p.Id()] = p
					}
				case "]":
					np, _ := wh.points[p.WestId()]
					if np.value != "[" {
						p.value = "."
						wh.points[p.Id()] = p
					}
				}
			}
		}
	}
}

// ---------- UTILITIES -----------------------------------

func (wh Warehouse) Dimensions() (int, int, int, int) {
	min_x := 999999999
	max_x := 0
	min_y := 999999999
	max_y := 0
	for _, p := range wh.points {
		switch {
		case p.x < min_x:
			min_x = p.x
		case p.x > max_x:
			max_x = p.x
		case p.y < min_y:
			min_y = p.y
		case p.y > max_y:
			max_y = p.y
		}
	}
	return min_x, max_x, min_y, max_y
}

func (wh Warehouse) ForwardOffsets(move string) (int, int) {
	dx := 0
	dy := 0
	switch move {
	case "^":
		dy = -1
	case ">":
		dx = 1
	case "v":
		dy = 1
	case "<":
		dx = -1
	}
	return dx, dy
}

func (wh Warehouse) ForwardPoints(robot Point, move string) []Point {
	points := []Point{robot}
	found := false
	cps := []Point{robot}
search:
	for !found {
		new_cps := make([]Point, 0)
		maybes := make([]bool, 0)
		for _, cp := range cps {
			nid := wh.NextId(cp, move)
			np, ok := wh.points[nid]
			if ok {
				switch np.value {
				case "#":
					break search // hit a wall
				case "O":
					points = append(points, np)
					new_cps = append(new_cps, np)
					maybes = append(maybes, false)
				case "[":
					ap, _ := wh.points[np.EastId()]
					points = append(points, np)
					points = append(points, ap)
					maybes = append(maybes, false)
					switch move {
					case ">":
						new_cps = append(new_cps, ap)
					case "<":
						// noop: should never happen
					default:
						new_cps = append(new_cps, np)
						new_cps = append(new_cps, ap)
					}
				case "]":
					ap, _ := wh.points[np.WestId()]
					points = append(points, np)
					points = append(points, ap)
					maybes = append(maybes, false)
					switch move {
					case "<":
						new_cps = append(new_cps, ap)
					case ">":
						// noop: should never happen
					default:
						new_cps = append(new_cps, np)
						new_cps = append(new_cps, ap)
					}
				case ".":
					maybes = append(maybes, true)
					// break search
				}
			} else {
				break search // off the map
			}
		}
		cps = new_cps
		if !pie.Contains(maybes, false) {
			found = true
		}
	}
	if !found {
		points = make([]Point, 0)
	}
	return points
}

func (wh Warehouse) GetRobot() Point {
	robot := Point{}
	for _, p := range wh.points {
		if p.IsRobot() {
			robot = p
			break
		}
	}
	return robot
}

func (wh Warehouse) NextId(p Point, move string) string {
	nid := ""
	switch move {
	case "^":
		nid = p.NorthId()
	case ">":
		nid = p.EastId()
	case "v":
		nid = p.SouthId()
	case "<":
		nid = p.WestId()
	}
	return nid
}

func (wh Warehouse) Print() {
	min_x, max_x, min_y, max_y := wh.Dimensions()

	for y := min_y; y <= max_y; y++ {
		row := ""
		for x := min_x; x <= max_x; x++ {
			p, _ := wh.points[utility.CoordToId(x, y)]
			row += p.value
		}
		fmt.Println(row)
	}
	fmt.Println("")
	fmt.Println("")
}

func (wh Warehouse) UnpackPoints(points []Point) ([]string, []string) {
	ids := make([]string, 0)
	vals := make([]string, 0)
	for _, p := range points {
		ids = append(ids, p.Id())
		vals = append(vals, p.value)
	}
	return ids, vals
}
