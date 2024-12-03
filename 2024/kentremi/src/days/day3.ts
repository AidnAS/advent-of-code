import type { FileHandle } from "fs/promises";

async function day3(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  const mulInstructions: string[] = [];
  for await (const line of dayFileHandle.readLines()) {
    const matchEx = /(mul\(\d{1,3},\d{1,3}\))/g;

    let match;
    while ((match = matchEx.exec(line)) !== null) {
      if (match.index === matchEx.lastIndex) {
        matchEx.lastIndex++;
      }

      const mulNumbers = match[1].match(/(\d{1,3}),(\d{1,3})/);

      if (mulNumbers) {
        mulInstructions.push(mulNumbers[0]);
      }
    }
  }

  mulInstructions.forEach((mul) => {
    const mulDigits = mul.split(",").map((m) => Number(m));
    const mulResult = mulDigits[0] * mulDigits[1];
    result += mulResult;
  });

  return result.toString();
}

export default day3;
