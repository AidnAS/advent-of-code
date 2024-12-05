import { PuzzleSolver } from './../types/index';
import { dataFileReader } from '../utils/index';


const solvePart1: PuzzleSolver = (date, useTestData = false) => {
	console.log('Day 03, Part 1: ');

	// to do: implementation
	const lines = dataFileReader({ date, useTestData, part: 1 });

	lines.map(line => console.log(line));

	console.log('The answer is...');
};

const solvePart2: PuzzleSolver = (date, useTestData = false) => {
	console.log('Day 03, Part 2: ');

	// to do: implementation

	console.log('The answer is...');
};

export default {
	solvePart1,
	solvePart2
}