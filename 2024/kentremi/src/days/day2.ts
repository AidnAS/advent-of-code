import type { FileHandle } from "fs/promises";

async function day2(task: number, dayFileHandle: FileHandle) {
  let result = 0;

  const reports: number[][] = [];
  for await (const line of dayFileHandle.readLines()) {
    const report = line.split(" ");
    reports.push(report.map((r) => Number(r)));
  }

  const checkReport = (report: number[]) => {
    let isSafe = true;
    let firstRun = true;

    const checkRep = (rep: number[], iteration: number) => {
      const isIncreasing = rep[0] < rep[1];
      rep.every((level, i) => {
        if (rep[i + 1] !== undefined) {
          const distance = isIncreasing
            ? rep[i + 1] - level
            : level - rep[i + 1];
          if (distance < 1 || distance > 3) {
            isSafe = false;
            return false;
          }
        }
        return true;
      });

      // Go recursive if task 2
      if (task == 2 && !isSafe && iteration < report.length) {
        isSafe = true;

        const damperedReport = report.filter(
          (_value, arrIndex) => iteration !== arrIndex
        );

        checkRep(damperedReport, (iteration += 1));
      }

      firstRun = false;
    };

    if (firstRun) {
      checkRep(report, 0);
    }

    return isSafe;
  };

  reports.forEach((report) => {
    let isSafe = checkReport(report);

    if (isSafe) {
      result += 1;
    }
  });

  return result.toString();
}

export default day2;
