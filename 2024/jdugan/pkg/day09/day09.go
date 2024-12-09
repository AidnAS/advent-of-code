package day09

import (
	"aoc/2024/pkg/reader"
	"fmt"
	"strings"

	"github.com/elliotchance/pie/v2"
)

// ========== PUBLIC FNS ==================================

func Both() {
	fmt.Println(" ")
	fmt.Println("DAY", 9)
	fmt.Println("  Puzzle 1", "=>", Puzzle1())
	fmt.Println("  Puzzle 2", "=>", Puzzle2())
	fmt.Println(" ")
}

func Puzzle1() int {
	fs := data()
	return fs.CompressBlocks()
}

func Puzzle2() int {
	fs := data()
	return fs.CompressFiles()
}

// ========== PRIVATE FNS =================================

func data() FileSystem {
	lines := reader.Lines("./data/day09/input.txt")
	line := lines[0] + "0"
	ints := pie.Ints(strings.Split(line, ""))
	chunks := pie.Chunk(ints, 2)

	files := make([]File, 0)
	for id, chunk := range chunks {
		file := File{id: id, block_size: chunk[0], free_size: chunk[1]}
		files = append(files, file)
	}
	return FileSystem{files: files}
}
