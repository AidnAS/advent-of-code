import {
	boilerplate,
	day01,
} from './puzzles';


const currentDate = (new Date()).toLocaleDateString('en-GB').split('/').join('-');

console.log(`AoC: ${currentDate}`);
console.log('-----------------\n');

// test boilerplate
const testDate = "01-12-2000";
boilerplate.solvePart1(testDate, true);
boilerplate.solvePart2(testDate);

// run puzzle solvers

// day01.solvePart1(currentDate, true);
// day01.solvePart2(currentDate);