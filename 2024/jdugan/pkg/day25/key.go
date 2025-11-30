package day25

// ========== DEFINITION ==================================

type Key struct {
	notches []int
}

// ========== RECEIVERS ===================================

func (k Key) Fits(lock Lock) bool {
	fits := true
	for i, kh := range k.notches {
		if kh+lock.pins[i] > lock.max {
			fits = false
			break
		}
	}
	return fits
}
