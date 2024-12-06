import type { FileHandle } from "fs/promises";

async function day5(task: number, dayFileHandle: FileHandle) {
  let result = 0;

  const pageRuleMap = new Map<string, string[]>();
  const pageUpdates = [];

  for await (const line of dayFileHandle.readLines()) {
    if (line.includes("|")) {
      const pageRule = line.split("|");

      pageRuleMap.has(pageRule[0])
        ? pageRuleMap.get(pageRule[0])?.push(pageRule[1])
        : pageRuleMap.set(pageRule[0], [pageRule[1]]);
    }

    if (line.includes(",")) {
      pageUpdates.push(line.split(","));
    }
  }

  let validUpdates: number[] = [];
  let invalidUpdates: string[][] = [];

  pageUpdates.forEach((pageUpdate) => {
    let updateIsValid = true;
    for (let i = 0; i < pageUpdate.length - 1; i++) {
      const checkIndex = pageUpdate[i];
      const checkValue = pageUpdate.at(i + 1) || "";
      const pageRuleRange = pageRuleMap.get(checkIndex);
      if (!pageRuleRange?.includes(checkValue)) {
        updateIsValid = false;
      }
    }

    if (updateIsValid) {
      if (task === 1) {
        const midPage = pageUpdate.at(pageUpdate.length / 2);
        validUpdates.push(Number(midPage));
      }
    } else {
      invalidUpdates.push(pageUpdate);
    }
  });

  if (task === 2) {
    invalidUpdates.forEach((invalidUpdate) => {
      const invUpdateRow = [...invalidUpdate];
      const sortedUpdateRow: number[] = [];

      let i = 0;
      while (sortedUpdateRow.length < invalidUpdate.length) {
        const invalid = invUpdateRow[i];
        const pageRuleRange = pageRuleMap.get(invalid);
        let isValidPos = true;
        invUpdateRow
          .filter((ivu) => ivu !== invalid)
          .every((rcr) => {
            if (!pageRuleRange?.includes(rcr)) {
              isValidPos = false;
              return false;
            }
            return true;
          });

        if (isValidPos) {
          sortedUpdateRow.push(Number(invalid));
          const ix = invUpdateRow.indexOf(invalid);
          const a = invUpdateRow.splice(ix, 1);
          i = 0;
        }

        if (i > invUpdateRow.length) {
          i = 0;
        } else {
          i++;
        }
      }

      const midPage = sortedUpdateRow.at(sortedUpdateRow.length / 2);
      validUpdates.push(Number(midPage));
    });
  }

  result = validUpdates.reduce((acc, curr) => {
    return acc + curr;
  });

  return result.toString();
}

export default day5;
