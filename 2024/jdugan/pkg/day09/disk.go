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

// ---------- HELPERS -------------------------------------

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

func (d Disk) SpaceIndices() []int {
	idxs := make([]int, 0)
	for i, v := range d.bytes {
		if v == -1 {
			idxs = append(idxs, i)
		}
	}
	return idxs
}

func (d Disk) SpaceIndicesForLength(length int) []int {
	idxs := make([]int, 0)
	prev := -2
	for i, v := range d.bytes {
		if v == -1 {
			if i-prev != 1 {
				idxs = make([]int, 0)
			}
			idxs = append(idxs, i)
			prev = i
			if len(idxs) == length {
				break
			}
		} else {
			idxs = make([]int, 0)
			prev = -2
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

func (d Disk) ValueIndicesById() map[int][]int {
	imap := make(map[int][]int, 0)
	for i, v := range d.bytes {
		if v != -1 {
			is, ok := imap[v]
			if ok {
				is = append(is, i)
				imap[v] = is
			} else {
				imap[v] = []int{i}
			}
		}
	}
	return imap
}
