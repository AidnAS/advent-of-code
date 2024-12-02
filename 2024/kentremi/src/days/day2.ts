import type { FileHandle } from "fs/promises";

async function day2(task: number, dayFileHandle: FileHandle) {
  let result = 0;

  const reports: number[][] = [];
  for await (const line of dayFileHandle.readLines()) {
    const report = line.split(" ");
    reports.push(report.map((r) => Number(r)));
  }

  reports.forEach((report) => {
    let isSafe = true;
    const isIncreasing = report[0] < report[1];

    report.forEach((level, i) => {
      if (report[i + 1] !== undefined) {
        const distance = isIncreasing
          ? report[i + 1] - level
          : level - report[i + 1];
        if (distance < 1 || distance > 3) {
          isSafe = false;
        }
      }
    });

    if (isSafe) {
      result += 1;
    }
  });

  return result.toString();
}

export default day2;
