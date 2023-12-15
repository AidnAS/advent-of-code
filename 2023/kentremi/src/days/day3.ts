import { FileHandle } from 'fs/promises';

interface NumbersMap {
  value: number;
  firstAdjPos: number;
  lastAdjPos: number;
}

async function day3(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  let resultLine = 0;
  const numbersMap = new Map();
  const symbolsMap = new Map();

  for await (const line of dayFileHandle.readLines()) {
    const numbersRegex = /(\d+)/g;
    const symbolsRegex = task === 1 ? /([^\d\.]+)/g : /(\*+)/g;

    numbersMap.set(resultLine, []);
    symbolsMap.set(resultLine, []);

    let numbers;
    while ((numbers = numbersRegex.exec(line)) !== null) {
      if (numbers.index === numbersRegex.lastIndex) {
        numbersRegex.lastIndex++;
      }

      numbersMap.set(resultLine, [
        ...numbersMap.get(resultLine),
        {
          value: Number(numbers[0]),
          firstAdjPos: numbers.index - 1,
          lastAdjPos: numbers.index + numbers[0].length,
        } as NumbersMap,
      ]);
    }

    let symbols;
    while ((symbols = symbolsRegex.exec(line)) !== null) {
      if (symbols.index === symbolsRegex.lastIndex) {
        symbolsRegex.lastIndex++;
      }

      symbolsMap.set(resultLine, [
        ...symbolsMap.get(resultLine),
        symbols.index,
      ]);
    }

    resultLine++;
  }

  const resultMap = new Map();

  const addToResultMap = (
    pos: number,
    row: number,
    key: number,
    value: Array<NumbersMap>
  ) => {
    value.forEach((val: NumbersMap) => {
      if (pos >= val.firstAdjPos && pos <= val.lastAdjPos) {
        resultMap.set(
          `${row}|${pos.toString()}|${key}|${val.firstAdjPos.toString()}|${val.lastAdjPos.toString()}`,
          val.value
        );
      }
    });
  };

  // Task 1 calculation
  numbersMap.forEach((value, key) => {
    const currSymbols = symbolsMap.get(key);
    const prevSymbols = symbolsMap.get(key - 1);
    const nextSymbols = symbolsMap.get(key + 1);

    currSymbols?.forEach((pos: number) => addToResultMap(pos, key, key, value));
    prevSymbols?.forEach((pos: number) =>
      addToResultMap(pos, key - 1, key, value)
    );
    nextSymbols?.forEach((pos: number) =>
      addToResultMap(pos, key + 1, key, value)
    );
  });

  // Task 2 calculation
  const matchPairs = new Map();
  const gearPairs: number[] = [];

  resultMap.forEach((value: number, key: string) => {
    const rowPosMatch = key.match(/(^\d+\|\d+)/);
    if (rowPosMatch) {
      if (matchPairs.has(rowPosMatch[0])) {
        gearPairs.push(matchPairs.get(rowPosMatch[0]) * value);
      }
      matchPairs.set(rowPosMatch[0], value);
    }
  });

  task === 1
    ? resultMap.forEach((value) => {
        result += value;
      })
    : gearPairs.forEach((val) => {
        result += val;
      });

  return result.toString();
}

export default day3;
