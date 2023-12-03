import { FileHandle } from 'fs/promises';
import { wordToNumber } from '../utils';

async function day1(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  for await (const line of dayFileHandle.readLines()) {
    const matchEx =
      task === 1
        ? /\d/g
        : /(?=(one|two|three|four|five|six|seven|eight|nine|\d))/g;

    const numbersInString = [];

    let match;
    while ((match = matchEx.exec(line)) !== null) {
      if (match.index === matchEx.lastIndex) {
        matchEx.lastIndex++;
      }

      numbersInString.push(task === 1 ? match[0] : match[1]);
    }

    const numberString =
      String(wordToNumber(numbersInString.slice(0)[0])) +
      String(wordToNumber(numbersInString.slice(-1)[0]));

    result += parseInt(numberString);
  }

  return result.toString();
}

export default day1;
