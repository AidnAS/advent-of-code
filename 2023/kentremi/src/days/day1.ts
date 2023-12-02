import { FileHandle } from 'fs/promises';
import { wordToNumber } from '../utils';

async function day1(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  for await (const line of dayFileHandle.readLines()) {
    const matchEx =
      task === 1 ? /\d/g : /(one|two|three|four|five|six|seven|eight|nine|\d)/g;

    const numbersInString = line.match(matchEx);

    if (numbersInString) {
      const numberString =
        String(wordToNumber(numbersInString.slice(0)[0])) +
        String(wordToNumber(numbersInString.slice(-1)[0]));

      result += parseInt(numberString);
    }
  }

  return result.toString();
}

export default day1;
