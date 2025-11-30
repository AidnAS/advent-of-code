package utility

import (
	"strconv"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== MATH HELPERS ================================

func CoordFromId(id string) (int, int) {
	parts := strings.Split(id, ",")
	i0, _ := strconv.Atoi(parts[0])
	i1, _ := strconv.Atoi(parts[1])
	return i0, i1
}

func CoordToId(x int, y int) string {
	sx := strconv.Itoa(x)
	sy := strconv.Itoa(y)
	return sx + "," + sy
}

func Distance(a int, b int) int {
	return pie.Abs(a - b)
}

func ReverseString(s string) string {
	bytes := make([]byte, 0)
	for i := len(s) - 1; i >= 0; i-- {
		bytes = append(bytes, s[i])
	}
	return string(bytes)
}
