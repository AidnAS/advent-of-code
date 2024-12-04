import type { FileHandle } from "fs/promises";

async function day4(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  const rowMap = new Map();

  let lineNumber = 0;
  for await (const line of dayFileHandle.readLines()) {
    const matchEx = /(X|M|A|S)/g;

    let match;

    const colMap = new Map();

    while ((match = matchEx.exec(line)) !== null) {
      if (match.index === matchEx.lastIndex) {
        matchEx.lastIndex++;
      }
      colMap.set(match.index, match[1]);
    }

    rowMap.set(lineNumber, colMap);
    lineNumber++;
  }

  let rowNum = 0;
  rowMap.forEach((row) => {
    const col = [...row.values()];
    for (let i = 0; i < col.length; i++) {
      // Check for horizontally in cols
      const horizontal = col.slice(i, i + 4).join("");
      if (horizontal === "XMAS" || horizontal === "SAMX") {
        result += 1;
      }

      // Check for diagonals and verticals
      if (rowNum < [...row].length - 3) {
        const diagonalLR =
          col[i] +
          rowMap.get(rowNum + 1).get(i + 1) +
          rowMap.get(rowNum + 2).get(i + 2) +
          rowMap.get(rowNum + 3).get(i + 3);

        if (diagonalLR === "XMAS" || diagonalLR === "SAMX") {
          result += 1;
        }

        const diagonalRL =
          col[col.length - 1 - i] +
          rowMap.get(rowNum + 1).get(col.length - 1 - i - 1) +
          rowMap.get(rowNum + 2).get(col.length - 1 - i - 2) +
          rowMap.get(rowNum + 3).get(col.length - 1 - i - 3);

        if (diagonalRL === "XMAS" || diagonalRL === "SAMX") {
          result += 1;
        }

        const vertical =
          col[i] +
          rowMap.get(rowNum + 1).get(i) +
          rowMap.get(rowNum + 2).get(i) +
          rowMap.get(rowNum + 3).get(i);

        if (vertical === "XMAS" || vertical === "SAMX") {
          result += 1;
        }
      }
    }
    rowNum += 1;
  });

  return result.toString();
}

export default day4;
