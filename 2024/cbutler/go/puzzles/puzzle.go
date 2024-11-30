package puzzles

type Puzzle struct {
	SolvePart1 func(date string, useTestData bool)
	SolvePart2 func(date string, useTestData bool)
}
