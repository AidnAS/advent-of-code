import type { FileHandle } from "fs/promises";

async function day1(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  const locationIds: number[][] = [[], []];
  for await (const line of dayFileHandle.readLines()) {
    const matchEx = /^(\d+)\s+(\d+)$/g;

    let match;
    while ((match = matchEx.exec(line)) !== null) {
      if (match.index === matchEx.lastIndex) {
        matchEx.lastIndex++;
      }

      locationIds[0].push(Number(match[1]));
      locationIds[1].push(Number(match[2]));
    }
  }

  if (task === 1) {
    const sortedLocationIds = [locationIds[0].sort(), locationIds[1].sort()];
    for (let i = 0; i < sortedLocationIds[0].length; i++) {
      const distance = Math.abs(
        sortedLocationIds[0][i] - sortedLocationIds[1][i]
      );
      result += distance;
    }
  } else {
    for (let i = 0; i < locationIds[0].length; i++) {
      let similarities = 0;
      for (let ii = 0; ii < locationIds[1].length; ii++) {
        if (locationIds[0][i] === locationIds[1][ii]) {
          similarities += 1;
        }
      }
      result += locationIds[0][i] * similarities;
    }
  }

  return result.toString();
}

export default day1;
