import { PuzzleSolver } from './../types/index';
import { dataFileReader } from '../utils/index';


const solvePart1: PuzzleSolver = (date, useTestData = false) => {
	console.log('Day 04, Part 1: XMAS');

	// read the data file
	const lines = dataFileReader({ date, useTestData, part: 1 });

	const matrix = createMatrix(lines);
	// printMatrix(matrix);

	const total = scanMatrix(matrix);

	var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
	console.log(`${outputString}: ${total}\n\n`);
};

const solvePart2: PuzzleSolver = (date, useTestData = false) => {
	console.log('Day 04, Part 2: X-MAS');

	// read the data file
	const lines = dataFileReader({ date, useTestData, part: 1 });

	const matrix = createMatrix(lines);
	// printMatrix(matrix);

	const total = turboScan(matrix);

	var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
	console.log(`${outputString}: ${total}\n\n`);
};

const createMatrix = (lines: string[]) => {
	const numRows = lines.length;
	const numCols = lines[0].length;
	const matrix: string[][] = new Array(numRows);

	for (let i = 0; i < numRows; i++) {
		matrix[i] = new Array(numCols);
		for (let j = 0; j < numCols; j++) {
			matrix[i][j] = lines[i].charAt(j);
		}
	}
	return matrix;
};

const scanMatrix = (matrix: string[][]) => {
	const numRows = matrix.length;
	const numCols = matrix[0].length;

	let count = 0;

	// pseudo code: Find XMAS
	// go char by char through the matrix.
	// check if the char is an X.
	// if yes, look in every direction to see if you can make the full word.
	// be aware of the edges of the matrix.

	for (let i = 0; i < numRows; i++) {
		for (let j = 0; j < numCols; j++) {
			if (matrix[i][j] === 'X') {
				// check up left
				if (i - 3 >= 0 && j - 3 >= 0) {
					if (matrix[i - 1][j - 1] === 'M' && matrix[i - 2][j - 2] === 'A' && matrix[i - 3][j - 3] === 'S') {
						count++;
					}
				}
				// check up
				if (i - 3 >= 0) {
					if (matrix[i - 1][j] === 'M' && matrix[i - 2][j] === 'A' && matrix[i - 3][j] === 'S') {
						count++;
					}
				}
				// check up right
				if (i - 3 >= 0 && j + 3 < numCols) {
					if (matrix[i - 1][j + 1] === 'M' && matrix[i - 2][j + 2] === 'A' && matrix[i - 3][j + 3] === 'S') {
						count++;
					}
				}
				// check right
				if (j + 3 < numCols) {
					if (matrix[i][j + 1] === 'M' && matrix[i][j + 2] === 'A' && matrix[i][j + 3] === 'S') {
						count++;
					}
				}
				// check down right
				if (i + 3 < numRows && j + 3 < numCols) {
					if (matrix[i + 1][j + 1] === 'M' && matrix[i + 2][j + 2] === 'A' && matrix[i + 3][j + 3] === 'S') {
						count++;
					}
				}
				// check down
				if (i + 3 < numRows) {
					if (matrix[i + 1][j] === 'M' && matrix[i + 2][j] === 'A' && matrix[i + 3][j] === 'S') {
						count++;
					}
				}
				// check down left
				if (i + 3 < numRows && j - 3 >= 0) {
					if (matrix[i + 1][j - 1] === 'M' && matrix[i + 2][j - 2] === 'A' && matrix[i + 3][j - 3] === 'S') {
						count++;
					}
				}
				// check left
				if (j - 3 >= 0) {
					if (matrix[i][j - 1] === 'M' && matrix[i][j - 2] === 'A' && matrix[i][j - 3] === 'S') {
						count++;
					}
				}
			}
		}
	}
	return count;
};

const turboScan = (matrix: string[][]) => {
	const numRows = matrix.length;
	const numCols = matrix[0].length;

	let count = 0;

	// psuedo code: Find X-MAS
	// go char by char through the matrix. (scanning down)
	// check if you can make the shape and if it is there. 
	// i.e. the char at [i,j] must be M or S to start the upper left corner of the shape.
	// be aware of the edges of the matrix again.
	// only count if ALL conditions are met.

	for (let i = 0; i < numRows; i++) {
		for (let j = 0; j < numCols; j++) {
			if (matrix[i][j] === 'M' || matrix[i][j] === 'S') {
				if (bfCheck(matrix, i, j)) {
					count++;
				}
			}
		}
	}
	return count;
};

function bfCheck (m: string[][], i: number, j: number) {
	const numRows = m.length;
	const numCols = m[0].length;


	// check if the shape can be made
	if (i+2 >= numRows || j+2 >= numCols) return false;

	// 1a. the letter is M
	// M.M
	// .A.
	// S.S
	var mm = m[i][j] == 'M' && m[i+1][j+1] == 'A' && m[i+2][j+2] == 'S' && m[i+2][j] == 'M' && m[i][j+2] == 'S';
	
	// 1b. the letter is M
	// M.S
	// .A.
	// M.S
	var ms = m[i][j] == 'M' && m[i+1][j+1] == 'A' && m[i+2][j+2] == 'S' && m[i+2][j] == 'S' && m[i][j+2] == 'M';

	// 2a. the letter is S
	// S.S
	// .A.
	// M.M
	var ss = m[i][j] == 'S' && m[i+1][j+1] == 'A' && m[i+2][j+2] == 'M' && m[i+2][j] == 'S' && m[i][j+2] == 'M';

	// 2b. the letter is S
	// S.M
	// .A.
	// S.M
	var sm = m[i][j] == 'S' && m[i+1][j+1] == 'A' && m[i+2][j+2] == 'M' && m[i+2][j] == 'M' && m[i][j+2] == 'S';

	return mm || ms || ss || sm;
};


const printMatrix = (matrix: string[][]) => {
	const numRows = matrix.length;
	const numCols = matrix[0].length;

	for (let i = 0; i < numRows; i++) {
		let rowString = '';
		for (let j = 0; j < numCols; j++) {
			rowString += `[${matrix[i][j]}]`;
		}
		console.log(rowString.trim());
	}
};

export default {
	solvePart1,
	solvePart2
}