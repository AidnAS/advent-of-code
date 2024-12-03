import type { FileHandle } from "fs/promises";

async function day3(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  const mulInstructions = new Map();

  let lineNumber = 0;
  for await (const line of dayFileHandle.readLines()) {
    const matchEx = /(mul\(\d{1,3},\d{1,3}\))/g;
    const doEx = /(do\(\))/g;
    const dontEx = /(don't\(\))/g;

    let match;
    const mulMap = new Map();

    while ((match = matchEx.exec(line)) !== null) {
      if (match.index === matchEx.lastIndex) {
        matchEx.lastIndex++;
      }

      const mulNumbers = match[1].match(/(\d{1,3}),(\d{1,3})/);

      if (mulNumbers) {
        mulMap.set(match.index, mulNumbers[0]);
      }
    }

    // Add dos and don'ts if task 2
    if (task === 2) {
      let dos;
      while ((dos = doEx.exec(line)) !== null) {
        if (dos.index === doEx.lastIndex) {
          doEx.lastIndex++;
        }
        mulMap.set(dos.index, "include");
      }

      let donts;
      while ((donts = dontEx.exec(line)) !== null) {
        if (donts.index === dontEx.lastIndex) {
          dontEx.lastIndex++;
        }
        mulMap.set(donts.index, "exclude");
      }
    }

    mulInstructions.set(lineNumber, mulMap);
    lineNumber++;
  }

  if (task === 1) {
    mulInstructions.forEach((mulMap) => {
      mulMap.forEach((mul: string) => {
        const mulDigits = mul.split(",").map((m: string) => Number(m));
        const mulResult = mulDigits[0] * mulDigits[1];
        result += mulResult;
      });
    });
  } else {
    const sortedMulInstructions = new Map(
      [...mulInstructions.entries()].sort()
    );

    let include = true;
    sortedMulInstructions.forEach((mulMap, key) => {
      const sortedMulMap = new Map(
        [...mulMap.entries()].sort((a, b) => a[0] - b[0])
      );

      sortedMulMap.forEach((mul, key) => {
        if (mul === "include") {
          console.log("include", key);
          include = true;
        }
        if (mul === "exclude") {
          console.log("exclude", key);
          include = false;
        }

        console.log(key, include);

        const mulDigits = String(mul)
          .split(",")
          .map((m: string) => Number(m));

        if (include && mulDigits.length === 2) {
          const mulResult = mulDigits[0] * mulDigits[1];
          result += mulResult;
        }
      });
    });
  }

  return result.toString();
}

export default day3;
