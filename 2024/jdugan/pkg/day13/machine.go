package day13

// ========== DEFINITION ==================================

type Machine struct {
	ax int
	ay int
	bx int
	by int
	px int
	py int
}

// ========== RECEIVERS ===================================

func (m Machine) FewestCoins(fa int, fb int) int {
	a_times, b_times := m.WinningCombo()
	return (fa * a_times) + (fb * b_times)
}

// ---------- UTILITIES -----------------------------------

func (m Machine) WinningCombo() (int, int) {
	num_a := (m.by*m.px - m.bx*m.py) / (m.by*m.ax - m.bx*m.ay)
	num_b := (m.py - m.ay*num_a) / m.by
	if m.ax*num_a+m.bx*num_b != m.px || m.ay*num_a+m.by*num_b != m.py {
		num_a = 0
		num_b = 0
	}
	return num_a, num_b
}
