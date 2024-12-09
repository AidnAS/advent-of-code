import type { FileHandle } from "fs/promises";

interface GuardLocation {
  row: number;
  col: number;
}

enum Direction {
  NORTH,
  EAST,
  SOUTH,
  WEST,
}

async function day6(task: number, dayFileHandle: FileHandle) {
  let result = 0;

  const rowMap = new Map<number, number[]>();
  const guardStartPos: GuardLocation = {
    row: 0,
    col: 0,
  };
  const guardVisits = new Set();
  let colLength = 0;

  let lineNumber = 0;
  for await (const line of dayFileHandle.readLines()) {
    if (colLength === 0) {
      colLength = line.length;
    }

    const matchEx = /(\#)|(\^)/g;

    let match;

    const obstacles = [];

    while ((match = matchEx.exec(line)) !== null) {
      if (match.index === matchEx.lastIndex) {
        matchEx.lastIndex++;
      }

      if (match[0] === "^") {
        guardStartPos.row = lineNumber;
        guardStartPos.col = match.index;
      }
      if (match[0] === "#") {
        obstacles.push(match.index);
      }
    }

    rowMap.set(lineNumber, obstacles);
    lineNumber++;
  }

  const go = (
    direction: Direction,
    steps: number,
    guardLocation: GuardLocation
  ): number => {
    let directionSteps = 0;
    let currGuardLocation: GuardLocation = guardLocation;
    let nextDirection: Direction = Direction.NORTH;
    let hasObstacle = false;

    while (true) {
      switch (direction) {
        case Direction.NORTH:
          currGuardLocation = {
            row: guardLocation.row - directionSteps,
            col: guardLocation.col,
          };
          nextDirection = Direction.EAST;
          if (currGuardLocation.row === 0) {
            return steps;
          }
          hasObstacle =
            rowMap
              .get(currGuardLocation.row - 1)
              ?.includes(currGuardLocation.col) || false;
          break;
        case Direction.EAST:
          currGuardLocation = {
            row: guardLocation.row,
            col: guardLocation.col + directionSteps,
          };
          nextDirection = Direction.SOUTH;
          if (currGuardLocation.col === colLength) {
            return steps;
          }
          hasObstacle =
            rowMap
              .get(currGuardLocation.row)
              ?.includes(currGuardLocation.col + 1) || false;
          break;
        case Direction.SOUTH:
          currGuardLocation = {
            row: guardLocation.row + directionSteps,
            col: guardLocation.col,
          };
          nextDirection = Direction.WEST;
          if (currGuardLocation.row === rowMap.size) {
            return steps;
          }
          hasObstacle =
            rowMap
              .get(currGuardLocation.row + 1)
              ?.includes(currGuardLocation.col) || false;
          break;
        case Direction.WEST:
          currGuardLocation = {
            row: guardLocation.row,
            col: guardLocation.col - directionSteps,
          };
          nextDirection = Direction.NORTH;
          if (currGuardLocation.col + 1 === 0) {
            return steps;
          }
          hasObstacle =
            rowMap
              .get(currGuardLocation.row)
              ?.includes(currGuardLocation.col - 1) || false;
          break;
      }

      if (hasObstacle) {
        return go(nextDirection, steps, currGuardLocation);
      } else {
        steps += 1;
        guardVisits.add(
          String(currGuardLocation.row) + "-" + String(currGuardLocation.col)
        );
      }

      directionSteps += 1;
    }
  };

  guardVisits.add(String(guardStartPos.row) + "-" + String(guardStartPos.col));

  go(Direction.NORTH, 0, guardStartPos);

  result = guardVisits.size;

  return result.toString();
}

export default day6;
