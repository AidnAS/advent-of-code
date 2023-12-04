import { FileHandle } from 'fs/promises';

async function day2(task: number, dayFileHandle: FileHandle) {
  let result = 0;
  for await (const line of dayFileHandle.readLines()) {
    const gameIdMatch = /Game (\d+)/.exec(line);
    const gamesMatch = /Game \d+:\s(.+)/.exec(line);

    if (!gameIdMatch || !gamesMatch) return 'DATA ERROR!';

    const gameId = parseInt(gameIdMatch[1]);
    const gamesData = gamesMatch[1].split(';');
    let invalid = false;
    let highestNumbers = { red: 0, green: 0, blue: 0 };
    const gameHighestNumbers = new Map();

    gamesData.forEach((gameData) => {
      gameData.split(',').every((game) => {
        const cubes = /(\d+)\s(red|green|blue)/.exec(game.trim());
        if (!cubes) return;

        const color = String(cubes[2]) as 'red' | 'green' | 'blue';
        const value = parseInt(cubes[1]);

        if (highestNumbers[color] < value) {
          highestNumbers[color] = value;
        }

        gameHighestNumbers.set(gameId, highestNumbers);

        if (task === 2) return true;

        switch (color) {
          case 'red':
            if (value > 12) {
              invalid = true;
            }
            break;
          case 'green':
            if (value > 13) {
              invalid = true;
            }
            break;
          case 'blue':
            if (value > 14) {
              invalid = true;
            }
            break;
        }

        if (invalid && task === 1) return false;
        return true;
      });
    });

    if (task === 1 && !invalid) {
      result += gameId;
    }

    if (task === 2) {
      gameHighestNumbers.forEach((nums) => {
        const gameResult = nums['red'] * nums['green'] * nums['blue'];
        result += gameResult;
      });
    }
  }

  return result.toString();
}

export default day2;
