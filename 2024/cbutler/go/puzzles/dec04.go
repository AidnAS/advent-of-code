package puzzles

import (
	"aoc/utils"
	"fmt"
	"strconv"
)

var Dec04 = Puzzle{
	SolvePart1: func(date string, useTestData bool) {
		fmt.Println("Day 04, Part 1: XMAS")

		lines, err := utils.DataFileReader(date, useTestData, "1", false)
		if err != nil {
			fmt.Println("Error reading file:", err)
			return
		}

		var matrix = CreateMatrix(lines)
		// PrintMatrix(matrix)

		var total = ScanMatrix(matrix)

		utils.PrintOutput(strconv.Itoa(total), useTestData)

	},

	SolvePart2: func(date string, useTestData bool) {
		fmt.Println("Day 04, Part 2: X-MAS")

		lines, err := utils.DataFileReader(date, useTestData, "1", false)
		if err != nil {
			fmt.Println("Error reading file:", err)
			return
		}

		var matrix = CreateMatrix(lines)
		// PrintMatrix(matrix)

		var total = TurboScan(matrix)

		utils.PrintOutput(strconv.Itoa(total), useTestData)
	},
}

func CreateMatrix(lines []string) [][]rune {
	var numRows = len(lines)
	var numCols = len([]rune(lines[0]))

	// println("numRows:", numRows, "numCols:", numCols)

	// rune is an alias for int32
	var matrix = make([][]rune, numRows)

	for i := 0; i < numRows; i++ {
		matrix[i] = make([]rune, numCols) // Initialize the inner slice
		for j := 0; j < numCols; j++ {
			matrix[i][j] = rune(lines[i][j])
		}
	}
	return matrix
}

func ScanMatrix(matrix [][]rune) int {
	var numRows = len(matrix)
	var numCols = len(matrix[0])

	var count = 0

	// pseudo code: Find XMAS
	// go char by char through the matrix.
	// check if the char is an X.
	// if yes, look in every direction to see if you can make the full word.
	// be aware of the edges of the matrix.

	for i := 0; i < numRows; i++ {
		for j := 0; j < numCols; j++ {
			if matrix[i][j] == 'X' {
				// check up left
				if i-3 >= 0 && j-3 >= 0 {
					if matrix[i-1][j-1] == 'M' && matrix[i-2][j-2] == 'A' && matrix[i-3][j-3] == 'S' {
						count++
					}
				}
				// check up
				if i-3 >= 0 {
					if matrix[i-1][j] == 'M' && matrix[i-2][j] == 'A' && matrix[i-3][j] == 'S' {
						count++
					}
				}
				// check up right
				if i-3 >= 0 && j+3 < numCols {
					if matrix[i-1][j+1] == 'M' && matrix[i-2][j+2] == 'A' && matrix[i-3][j+3] == 'S' {
						count++
					}
				}
				// check right
				if j+3 < numCols {
					if matrix[i][j+1] == 'M' && matrix[i][j+2] == 'A' && matrix[i][j+3] == 'S' {
						count++
					}
				}
				// check down right
				if i+3 < numRows && j+3 < numCols {
					if matrix[i+1][j+1] == 'M' && matrix[i+2][j+2] == 'A' && matrix[i+3][j+3] == 'S' {
						count++
					}
				}
				// check down
				if i+3 < numRows {
					if matrix[i+1][j] == 'M' && matrix[i+2][j] == 'A' && matrix[i+3][j] == 'S' {
						count++
					}
				}
				// check down left
				if i+3 < numRows && j-3 >= 0 {
					if matrix[i+1][j-1] == 'M' && matrix[i+2][j-2] == 'A' && matrix[i+3][j-3] == 'S' {
						count++
					}
				}
				// check left
				if j-3 >= 0 {
					if matrix[i][j-1] == 'M' && matrix[i][j-2] == 'A' && matrix[i][j-3] == 'S' {
						count++
					}
				}
			}
		}
	}

	return count
}

func TurboScan(matrix [][]rune) int {
	var numRows = len(matrix)
	var numCols = len(matrix[0])

	var count = 0

	// psuedo code: Find X-MAS
	// go char by char through the matrix. (scanning down)
	// check if you can make the shape and if it is there.
	// i.e. the char at [i,j] must be M or S to start the upper left corner of the shape.
	// be aware of the edges of the matrix again.
	// only count if ALL conditions are met.

	for i := 0; i < numRows; i++ {
		for j := 0; j < numCols; j++ {
			if matrix[i][j] == 'M' || matrix[i][j] == 'S' {
				if BFCheck(matrix, i, j) {
					count++
				}
			}
		}
	}
	return count
}

func BFCheck(m [][]rune, i int, j int) bool {
	var numRows = len(m)
	var numCols = len(m[0])

	// check if the shape can be made
	if i+2 >= numRows || j+2 >= numCols {
		return false
	}

	// 1a. the letter is M
	// M.M
	// .A.
	// S.S
	var mm = m[i][j] == 'M' && m[i+1][j+1] == 'A' && m[i+2][j+2] == 'S' && m[i+2][j] == 'M' && m[i][j+2] == 'S'

	// 1b. the letter is M
	// M.S
	// .A.
	// M.S
	var ms = m[i][j] == 'M' && m[i+1][j+1] == 'A' && m[i+2][j+2] == 'S' && m[i+2][j] == 'S' && m[i][j+2] == 'M'

	// 2a. the letter is S
	// S.S
	// .A.
	// M.M
	var ss = m[i][j] == 'S' && m[i+1][j+1] == 'A' && m[i+2][j+2] == 'M' && m[i+2][j] == 'S' && m[i][j+2] == 'M'

	// 2b. the letter is S
	// S.M
	// .A.
	// S.M
	var sm = m[i][j] == 'S' && m[i+1][j+1] == 'A' && m[i+2][j+2] == 'M' && m[i+2][j] == 'M' && m[i][j+2] == 'S'

	return mm || ms || ss || sm
}

func PrintMatrix(matrix [][]rune) {
	var numRows = len(matrix)
	var numCols = len(matrix[0])

	for i := 0; i < numRows; i++ {
		for j := 0; j < numCols; j++ {
			fmt.Printf(" [%c] ", matrix[i][j])
		}
		fmt.Println()
	}
}
