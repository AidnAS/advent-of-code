import { FileHandle } from 'fs/promises';

async function day4(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  let lineNumber = 0;
  const cardHitInstances = new Map();

  for await (const line of dayFileHandle.readLines()) {
    const cardRegex = /.+(\d+):(.+)\|(.+)/g;

    let winnerNumbers: number[] = [];
    let myNumbers: number[] = [];

    let cardLine;
    while ((cardLine = cardRegex.exec(line)) !== null) {
      if (cardLine.index === cardRegex.lastIndex) {
        cardRegex.lastIndex++;
      }
      winnerNumbers.push(
        ...cardLine[2]
          .trim()
          .split(/\s+/)
          .map((wn) => parseInt(wn))
      );
      myNumbers.push(
        ...cardLine[3]
          .trim()
          .split(/\s+/)
          .map((mn) => parseInt(mn))
      );
    }

    let numberOfHits = 0;
    myNumbers.forEach((myNumber, i) => {
      if (winnerNumbers.includes(myNumber)) {
        numberOfHits += 1;
      }
    });

    if (task === 1) {
      let hitResult = 0;
      for (let i = 0; i < numberOfHits; i++) {
        if (i === 0) {
          hitResult += 1;
        } else {
          hitResult = hitResult * 2;
        }
      }
      result += hitResult;
    } else {
      for (let hit = 0; hit < numberOfHits; hit++) {
        const nextRowNum = lineNumber + hit + 1;
        const thisLineCopies = cardHitInstances.get(lineNumber) || 0;
        const nextLineCopies = cardHitInstances.get(nextRowNum) || 1;

        if (thisLineCopies <= 1) {
          cardHitInstances.set(nextRowNum, nextLineCopies + 1);
        } else {
          cardHitInstances.set(nextRowNum, nextLineCopies + thisLineCopies);
        }
      }

      // If last card has no hit, we still have a card
      if (!cardHitInstances.has(lineNumber)) {
        cardHitInstances.set(lineNumber, 1);
      }
    }

    lineNumber += 1;
  }

  if (task === 2) {
    [...cardHitInstances.values()].forEach((cardHitInstance) => {
      result += cardHitInstance;
    });
  }

  return result.toString();
}

export default day4;
