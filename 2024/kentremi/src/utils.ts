import chalk from "chalk";
import * as fsPromise from "fs/promises";

export const printResult = (dayTask: string, result: string) => {
  console.log(
    chalk.red(`The solution for dayTask ${dayTask} is: `) + chalk.green(result)
  );
};

export const printExectime = (nanosec: bigint) => {
  const ms = Math.round(Number(Number(nanosec) / 1e6) * 100) / 100;
  console.log(chalk.yellow(`Execution time: ${ms} ms`));
};

export const getFileHandle = async (
  dayTask: string,
  isTest: boolean = false
) => {
  const fileName = isTest
    ? `./data/day${dayTask}.test`
    : `./data/day${dayTask.charAt(0)}.txt`;

  const dayFileHandle = await fsPromise.open(fileName, "r");
  return dayFileHandle;
};
