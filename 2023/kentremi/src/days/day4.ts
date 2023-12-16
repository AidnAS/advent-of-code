import { FileHandle } from 'fs/promises';

async function day4(task: number, dayFileHandle: FileHandle) {
  let result = 0;

  for await (const line of dayFileHandle.readLines()) {
    const cardRegex = /.+(\d+):(.+)\|(.+)/g;

    const winnerNumbers: number[][] = [];
    const myNumbers: number[][] = [];

    let cardLine;
    while ((cardLine = cardRegex.exec(line)) !== null) {
      if (cardLine.index === cardRegex.lastIndex) {
        cardRegex.lastIndex++;
      }
      winnerNumbers.push(
        cardLine[2]
          .trim()
          .split(/\s+/)
          .map((wn) => parseInt(wn))
      );
      myNumbers.push(
        cardLine[3]
          .trim()
          .split(/\s+/)
          .map((mn) => parseInt(mn))
      );
    }

    let numberOfHits = 0;
    myNumbers[0].forEach((myNumber) => {
      if (winnerNumbers[0].includes(myNumber)) {
        numberOfHits += 1;
      }
    });

    let hitResult = 0;
    for (let i = 0; i < numberOfHits; i++) {
      if (i === 0) {
        hitResult += 1;
      } else {
        hitResult = hitResult * 2;
      }
    }
    result += hitResult;
  }

  return result.toString();
}

export default day4;
