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
    const symbolsRegex = /([^\d\.]+)/g;

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
    key: number,
    value: Array<NumbersMap>
  ) => {
    value.forEach((val: NumbersMap) => {
      if (pos >= val.firstAdjPos && pos <= val.lastAdjPos) {
        resultMap.set(
          key.toString() +
            val.firstAdjPos.toString() +
            val.lastAdjPos.toString(),
          val.value
        );
      }
    });
  };

  numbersMap.forEach((value, key) => {
    const currSymbols = symbolsMap.get(key);
    const prevSymbols = symbolsMap.get(key - 1);
    const nextSymbols = symbolsMap.get(key + 1);

    currSymbols?.forEach((pos: number) => addToResultMap(pos, key, value));
    prevSymbols?.forEach((pos: number) => addToResultMap(pos, key, value));
    nextSymbols?.forEach((pos: number) => addToResultMap(pos, key, value));
  });

  resultMap.forEach((value) => {
    result += value;
  });

  return result.toString();
}

export default day3;
