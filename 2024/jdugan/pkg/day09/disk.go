package day09

import (
	"fmt"
	"strconv"
)

// ========== DEFINITION ==================================

type Disk struct {
	bytes []int
}

// ========== RECEIVERS ===================================

func (d Disk) CheckSum() int {
	checksum := 0
	for i, v := range d.bytes {
		if v > -1 {
			checksum += i * v
		}
	}
	return checksum
}

func (d Disk) SpaceIndices() []int {
	idxs := make([]int, 0)
	for i, v := range d.bytes {
		if v == -1 {
			idxs = append(idxs, i)
		}
	}
	return idxs
}

func (d Disk) SpaceIndicesByLength(length int) []int {
	spaces := d.SpaceIndices()
	idxs := make([]int, 0)
	prev := -2
	for _, v := range spaces {
		if v-prev != 1 {
			idxs = make([]int, 0)
		}
		idxs = append(idxs, v)
		prev = v
		if len(idxs) == length {
			break
		}
	}
	if len(idxs) != length {
		idxs = make([]int, 0)
	}
	return idxs
}

func (d Disk) ValueIndices() []int {
	idxs := make([]int, 0)
	for i, v := range d.bytes {
		if v > -1 {
			idxs = append(idxs, i)
		}
	}
	return idxs
}

func (d Disk) ValueIndicesForId(id int) []int {
	idxs := make([]int, 0)
	for i, v := range d.bytes {
		if v == id {
			idxs = append(idxs, i)
		}
	}
	return idxs
}

// ---------- UTILITIES -----------------------------------

func (d Disk) Print() {
	str := ""
	for _, v := range d.bytes {
		if v == -1 {
			str += "."
		} else {
			str += strconv.Itoa(v)
		}
	}
	fmt.Println(str)
}
