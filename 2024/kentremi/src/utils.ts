import chalk from "chalk";

export const printResult = (dayTask: string, result: string) => {
  console.log(
    chalk.red(`The solution for dayTask ${dayTask} is: `) + chalk.green(result)
  );
};

export const printExectime = (nanosec: bigint) => {
  const ms = Math.round(Number(Number(nanosec) / 1e6) * 100) / 100;
  console.log(chalk.yellow(`Execution time: ${ms} ms`));
};
