import { PuzzleSolver } from './../types/index';
import { dataFileReader } from '../utils/index';


const solvePart1: PuzzleSolver = (date, useTestData = false) => {
	console.log('Day 01, Part 1');

	const lines = dataFileReader({ date, useTestData, part: 1 });

	// console.log('lines: ');
	// lines.map(line => console.log(line));
	// console.log('--------------------\n');

	// create and sort lists
	const firstIds = lines.map(line => parseInt(line.split(' ')[0])).sort();
	const secondIds = lines.map(line => parseInt(line.split(' ').filter(part => part !== '')[1])).sort();

	// compare both lists and count
	if (firstIds.length !== secondIds.length) {
		console.log('Error: lists are not the same length');
		return;
	}

	const diffs: number[]  = [];

	for (let i = 0; i < firstIds.length; i++) {
		diffs.push(firstIds[i] >= secondIds[i] ? (firstIds[i] - secondIds[i]) : (secondIds[i] - firstIds[i]));
	}

	const total = diffs.reduce((acc, curr) => acc + curr, 0);

	var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
	console.log(`${outputString}: ${total}\n\n`);

	
};

const solvePart2: PuzzleSolver = (date, useTestData = false) => {
	console.log('Day 01, Part 2');

	const lines = dataFileReader({ date, useTestData, part: 1 });

		// create lists
		const firstIds = lines.map(line => parseInt(line.split(' ')[0]));
		const secondIds = lines.map(line => parseInt(line.split(' ').filter(part => part !== '')[1]));

		// compare both lists and count
		if (firstIds.length !== secondIds.length) {
			console.log('Error: lists are not the same length');
			return;
		}

		const scores: number[] = [];

		for (let i = 0; i < firstIds.length; i++) {
			let count = 0;
			for (let j = 0; j < secondIds.length; j++) {
				if (firstIds[i] === secondIds[j]) {
					count++;
				}
			}
			scores.push(firstIds[i] * count);
		}

		const total = scores.reduce((acc, curr) => acc + curr, 0);

		var outputString = useTestData ? "Total [using test data]" : "Total [using puzzle data]";
		console.log(`${outputString}: ${total}\n\n`);

};

export default {
	solvePart1,
	solvePart2
}