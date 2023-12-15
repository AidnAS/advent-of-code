import * as fsPromise from 'fs/promises';

import { Command } from 'commander';
import day1 from './days/day1';
import day2 from './days/day2';
import day3 from './days/day3';
import figlet from 'figlet';
import gradient from 'gradient-string';
import { printResult } from './utils';

const program = new Command();

const christmasGradient = gradient('red', 'green');
console.log(christmasGradient(figlet.textSync('AOC 2023')));

program
  .version('1.0.0')
  .description('CLI for Advent Of Code')
  .option(
    '-d, --day  [value]',
    "Execute [value] date's solution. Format <day-task>, e.g. 1-2"
  )
  .option(
    '-t, --test  [value]',
    "Execute [value] date's solution with test data. Format <day-task>, e.g. 1-2"
  )
  .parse(process.argv);

const options = program.opts();

async function executeDay(dayTask: string, isTest: boolean = false) {
  const fileName = isTest
    ? `./data/day${dayTask}.test`
    : `./data/day${dayTask.charAt(0)}.txt`;

  const dayFileHandle = await fsPromise.open(fileName, 'r');

  switch (dayTask) {
    case '1-1':
      return printResult('1-1', await day1(1, dayFileHandle));
    case '1-2':
      return printResult('1-2', await day1(2, dayFileHandle));
    case '2-1':
      return printResult('2-1', await day2(1, dayFileHandle));
    case '2-2':
      return printResult('2-2', await day2(2, dayFileHandle));
    case '3-1':
      return printResult('3-1', await day3(1, dayFileHandle));
    case '3-2':
      return printResult('3-2', await day3(2, dayFileHandle));
    default:
      console.log(christmasGradient(`Daytask ${dayTask} is not ready yet!`));
  }
}

if (options.day) {
  executeDay(options.day);
}

if (options.test) {
  executeDay(options.test, true);
}
