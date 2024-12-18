import { Command } from "commander";
import figlet from "figlet";
import gradient from "gradient-string";
import day1 from "./days/day1";
import day2 from "./days/day2";
import day3 from "./days/day3";
import day4 from "./days/day4";
import day5 from "./days/day5";
import day6 from "./days/day6";
import { getFileHandle, printExectime, printResult } from "./utils";

const program = new Command();

const christmasGradient = gradient("red", "green");
console.log(christmasGradient(figlet.textSync("AOC 2024")));

program
  .version("1.0.0")
  .description("CLI for Advent Of Code")
  .option(
    "-d, --day  [value]",
    "Execute [value] date's solution. Format <day-task>, e.g. 1-2"
  )
  .option(
    "-t, --test  [value]",
    "Execute [value] date's solution with test data. Format <day-task>, e.g. 1-2"
  )
  .parse(process.argv);

const options = program.opts();

async function executeDay(dayTask: string, isTest: boolean = false) {
  const dayFileHandle = await getFileHandle(dayTask, isTest);

  const startExec = process.hrtime.bigint(); // Date.now();

  switch (dayTask) {
    case "1-1":
      printResult("1-1", await day1(1, dayFileHandle));
      break;
    case "1-2":
      printResult("1-2", await day1(2, dayFileHandle));
      break;
    case "2-1":
      printResult("2-1", await day2(1, dayFileHandle));
      break;
    case "2-2":
      printResult("2-2", await day2(2, dayFileHandle));
      break;
    case "3-1":
      printResult("3-1", await day3(1, dayFileHandle));
      break;
    case "3-2":
      printResult("3-2", await day3(2, dayFileHandle));
      break;
    case "4-1":
      printResult("4-1", await day4(1, dayFileHandle));
      break;
    case "4-2":
      printResult("4-2", await day4(2, dayFileHandle));
      break;
    case "5-1":
      printResult("5-1", await day5(1, dayFileHandle));
      break;
    case "5-2":
      printResult("5-2", await day5(2, dayFileHandle));
      break;
    case "6-1":
      printResult("6-1", await day6(1, dayFileHandle));
      break;
    case "6-2":
      printResult("6-2", await day6(2, dayFileHandle));
      break;
    default:
      console.log(christmasGradient(`Daytask ${dayTask} is not ready yet!`));
  }

  const endExec = process.hrtime.bigint();
  printExectime(endExec - startExec);
}

if (options.day) {
  executeDay(options.day);
}

if (options.test) {
  executeDay(options.test, true);
}
