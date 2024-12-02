import { FileHandle } from "fs/promises";
import { wordToNumber } from "../utils";

async function day1(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  const locationIds: number[][] = [[], []];
  for await (const line of dayFileHandle.readLines()) {
    const matchEx = task === 1 ? /^(\d+)\s+(\d+)$/g : /todo/g;

    let match;
    while ((match = matchEx.exec(line)) !== null) {
      if (match.index === matchEx.lastIndex) {
        matchEx.lastIndex++;
      }

      locationIds[0].push(Number(match[1]));
      locationIds[1].push(Number(match[2]));
    }
  }

  const sortedLocationIds = [locationIds[0].sort(), locationIds[1].sort()];

  for (let i = 0; i < sortedLocationIds[0].length; i++) {
    const distance = Math.abs(
      sortedLocationIds[0][i] - sortedLocationIds[1][i]
    );

    result += distance;
  }

  return result.toString();
}

export default day1;
