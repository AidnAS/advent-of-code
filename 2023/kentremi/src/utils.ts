import chalk from 'chalk';

function printResult(dayTask: string, result: string) {
  console.log(
    chalk.red(`The solution for dayTask ${dayTask} is: `) + chalk.green(result)
  );
}

function wordToNumber(wordNumber: string) {
  if (!isNaN(parseInt(wordNumber))) {
    return wordNumber;
  }

  switch (wordNumber) {
    case 'one':
      return 1;
    case 'two':
      return 2;
    case 'three':
      return 3;
    case 'four':
      return 4;
    case 'five':
      return 5;
    case 'six':
      return 6;
    case 'seven':
      return 7;
    case 'eight':
      return 8;
    case 'nine':
      return 9;
  }
}

export { printResult, wordToNumber };
